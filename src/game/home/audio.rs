use crate::cpu::{Cpu, CpuFlag};

pub fn terminate_exp_bar_sound(cpu: &mut Cpu) {
    eprintln!("terminate_exp_bar_sound()");

    cpu.mmu.sound2.stop_sfx();

    // Run GameBoy code as well so that everything works like normally
    // xor a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);
}
