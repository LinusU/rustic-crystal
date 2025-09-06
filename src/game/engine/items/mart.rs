use crate::{
    cpu::Cpu,
    game::{
        constants::{
            mart_constants::{Mart, MartType},
            ram_constants::{DailyFlags, StatusFlags},
        },
        data::items::{bargain_shop, marts::MARTS, rooftop_sale},
    },
};

pub fn open_mart_dialog(cpu: &mut Cpu) {
    let mart_type = MartType::from(cpu.c);
    let mart = Mart::from(cpu.e);

    log::info!("open_mart_dialog({mart_type:?}, {mart:?})");

    cpu.borrow_wram_mut().set_mart_type(mart_type);

    get_mart(cpu, mart);
    cpu.call(0x5b10); // LoadMartPointer

    cpu.a = cpu.borrow_wram().mart_type().into();
    cpu.set_hl(0x5a57); // MartTypeDialogs
    cpu.call(0x0028); // JumpTable

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn bargain_shop(cpu: &mut Cpu) {
    cpu.b = bargain_shop::BARGAIN_SHOP_DATA.0;
    cpu.set_de(bargain_shop::BARGAIN_SHOP_DATA.1);
    cpu.call(0x5b10); // LoadMartPointer
    cpu.call(0x5c25); // ReadMart
    cpu.call(0x1d6e); // LoadStandardMenuHeader

    cpu.set_hl(0x5e6d); // BargainShopIntroText
    cpu.call(0x5fcd); // MartTextbox

    cpu.call(0x5c62); // BuyMenu

    if cpu.borrow_wram().bargain_shop_flags() != 0 {
        let mut flags = cpu.borrow_wram().daily_flags();
        flags.insert(DailyFlags::GOLDENROD_UNDERGROUND_BARGAIN);
        cpu.borrow_wram_mut().set_daily_flags(flags);
    }

    cpu.set_hl(0x5e8b); // BargainShopComeAgainText
    cpu.call(0x5fcd); // MartTextbox

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn rooftop_sale(cpu: &mut Cpu) {
    if !cpu
        .borrow_wram()
        .status_flags()
        .contains(StatusFlags::HALL_OF_FAME)
    {
        cpu.b = rooftop_sale::ROOFTOP_SALE_MART_1.0;
        cpu.set_de(rooftop_sale::ROOFTOP_SALE_MART_1.1);
    } else {
        cpu.b = rooftop_sale::ROOFTOP_SALE_MART_2.0;
        cpu.set_de(rooftop_sale::ROOFTOP_SALE_MART_2.1);
    }

    cpu.call(0x5b10); // LoadMartPointer
    cpu.call(0x5c25); // ReadMart
    cpu.call(0x1d6e); // LoadStandardMenuHeader

    cpu.set_hl(0x5f83); // MartWelcomeText
    cpu.call(0x5fcd); // MartTextbox

    cpu.call(0x5c62); // BuyMenu

    cpu.set_hl(0x5fb4); // MartComeAgainText
    cpu.call(0x5fcd); // MartTextbox

    cpu.pc = cpu.stack_pop(); // ret
}

fn get_mart(cpu: &mut Cpu, mart: Mart) {
    match mart {
        Mart::Unknown(_) => {
            cpu.b = 0x05; // BANK(DefaultMart)
            cpu.set_de(0x6214); // DefaultMart
        }

        _ => {
            cpu.b = 0x05; // BANK(Marts)
            cpu.set_de(MARTS[u8::from(mart) as usize]);
        }
    }
}
