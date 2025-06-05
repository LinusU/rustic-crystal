use crate::cpu::Cpu;

pub fn farcall(cpu: &mut Cpu, bank: u8, addr: u16) {
    // ld a, BANK(\1)
    cpu.a = bank;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld hl, \1
    cpu.set_hl(addr);
    cpu.pc += 3;
    cpu.cycle(12);

    // rst FarCall
    {
        cpu.pc += 1;
        let pc = cpu.pc;
        cpu.cycle(16);
        cpu.call(0x0008); // FarCall
        cpu.pc = pc;
    }
}

pub fn callfar(cpu: &mut Cpu, bank: u8, addr: u16) {
    // ld hl, \1
    cpu.set_hl(addr);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, BANK(\1)
    cpu.a = bank;
    cpu.pc += 2;
    cpu.cycle(8);

    // rst FarCall
    {
        cpu.pc += 1;
        let pc = cpu.pc;
        cpu.cycle(16);
        cpu.call(0x0008); // FarCall
        cpu.pc = pc;
    }
}
