use crate::{cpu::Cpu, game::ram::hram};

pub fn start(cpu: &mut Cpu) {
    eprintln!("start()");

    cpu.write_byte(hram::H_CGB, 1);
    cpu.write_byte(hram::H_SYSTEM_BOOTED, 1);
    cpu.a = 1;

    cpu.pc = 0x017d; // Init
}
