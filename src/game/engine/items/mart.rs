use crate::{
    cpu::Cpu,
    game::{
        constants::{mart_constants::Mart, ram_constants::StatusFlags},
        data::items::{marts::MARTS, rooftop_sale},
    },
};

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

pub fn get_mart(cpu: &mut Cpu) {
    let mart = Mart::from(cpu.e);

    log::info!("get_mart({mart:?}");

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

    cpu.pc = cpu.stack_pop(); // ret
}
