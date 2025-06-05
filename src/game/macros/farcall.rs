use crate::cpu::Cpu;

pub fn farcall(cpu: &mut Cpu, bank: u8, addr: u16) {
    // ld b, BANK(\1)
    cpu.b = bank;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld hl, \1
    cpu.set_hl(addr);
    cpu.pc += 3;
    cpu.cycle(12);

    // rst Bankswitch
    {
        cpu.pc += 1;
        let pc = cpu.pc;
        cpu.cycle(16);
        cpu.call(0x0010); // Bankswitch
        cpu.pc = pc;
    }
}

pub fn callfar(cpu: &mut Cpu, bank: u8, addr: u16) {
    // ld hl, \1
    cpu.set_hl(addr);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld b, BANK(\1)
    cpu.b = bank;
    cpu.pc += 2;
    cpu.cycle(8);

    // rst Bankswitch
    {
        cpu.pc += 1;
        let pc = cpu.pc;
        cpu.cycle(16);
        cpu.call(0x0010); // Bankswitch
        cpu.pc = pc;
    }
}
