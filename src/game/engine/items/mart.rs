use crate::{
    cpu::Cpu,
    game::{constants::mart_constants::Mart, data::items::marts::MARTS},
};

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
