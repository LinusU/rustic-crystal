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

    let ptr = get_mart(mart);
    load_mart_pointer(cpu, ptr);

    match mart_type {
        MartType::Standard => cpu.call(0x5a61), // MartDialog
        MartType::Bitter => cpu.call(0x5a6e),   // HerbShop
        MartType::Bargain => bargain_shop(cpu),
        MartType::Pharmacy => cpu.call(0x5aae), // Pharmacist
        MartType::Rooftop => rooftop_sale(cpu),
        MartType::Unknown(n) => unreachable!("Invalid mart type: {n}"),
    }

    cpu.pc = cpu.stack_pop(); // ret
}

fn bargain_shop(cpu: &mut Cpu) {
    load_mart_pointer(cpu, bargain_shop::BARGAIN_SHOP_DATA);
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
}

fn rooftop_sale(cpu: &mut Cpu) {
    let ptr = if !cpu
        .borrow_wram()
        .status_flags()
        .contains(StatusFlags::HALL_OF_FAME)
    {
        rooftop_sale::ROOFTOP_SALE_MART_1
    } else {
        rooftop_sale::ROOFTOP_SALE_MART_2
    };

    load_mart_pointer(cpu, ptr);
    cpu.call(0x5c25); // ReadMart
    cpu.call(0x1d6e); // LoadStandardMenuHeader

    cpu.set_hl(0x5f83); // MartWelcomeText
    cpu.call(0x5fcd); // MartTextbox

    cpu.call(0x5c62); // BuyMenu

    cpu.set_hl(0x5fb4); // MartComeAgainText
    cpu.call(0x5fcd); // MartTextbox
}

fn load_mart_pointer(cpu: &mut Cpu, ptr: (u8, u16)) {
    cpu.borrow_wram_mut().set_mart_pointer(ptr);

    cpu.a = 0;
    cpu.set_bc(16);
    cpu.set_hl(0xd0f0); // wCurMartCount
    cpu.call(0x3041); // ByteFill: fill bc bytes with the value of a, starting at hl

    cpu.borrow_wram_mut().set_mart_jumptable_index(0); // STANDARDMART_HOWMAYIHELPYOU
    cpu.borrow_wram_mut().set_bargain_shop_flags(0);
    cpu.a = 0;
}

fn get_mart(mart: Mart) -> (u8, u16) {
    match mart {
        Mart::Unknown(_) => {
            (0x05, 0x6214) // BANK(DefaultMart), DefaultMart
        }

        _ => {
            (0x05, MARTS[u8::from(mart) as usize]) // BANK(Marts)
        }
    }
}
