use crate::cpu::{Cpu, CpuFlag};

pub fn try_load_save_data(cpu: &mut Cpu) {
    eprintln!("try_load_save_data()");

    cpu.pc = 0x4f1c;

    // FALSE
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wSaveFileExists], a
    cpu.borrow_wram_mut().set_save_file_exists(false);
    cpu.pc += 3;
    cpu.cycle(16);

    // call CheckPrimarySaveFile
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x4f84); // CheckPrimarySaveFile
        cpu.pc = pc;
    }

    // ld a, [wSaveFileExists]
    cpu.a = cpu.borrow_wram_mut().save_file_exists() as u8;
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .backup
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return try_load_save_data_backup(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, BANK(sPlayerData)
    cpu.a = 1; // BANK(sPlayerData)
    cpu.pc += 2;
    cpu.cycle(8);

    // call OpenSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fcb); // OpenSRAM
        cpu.pc = pc;
    }

    // ld hl, sPlayerData + wStartDay - wPlayerData
    cpu.set_hl(0xa009 + (0xd4b6 - 0xd47b));
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wStartDay
    cpu.set_de(0xd4b6);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, 8
    cpu.set_bc(8);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyBytes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3026); // CopyBytes
        cpu.pc = pc;
    }

    // ld hl, sPlayerData + wStatusFlags - wPlayerData
    cpu.set_hl(0xa009 + (0xd84c - 0xd47b));
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wStatusFlags
    cpu.set_de(0xd84c);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // call CloseSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fe1); // CloseSRAM
        cpu.pc = pc;
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn try_load_save_data_backup(cpu: &mut Cpu) {
    cpu.pc = 0x4f46;

    // call CheckBackupSaveFile
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x4faf); // CheckBackupSaveFile
        cpu.pc = pc;
    }

    // ld a, [wSaveFileExists]
    cpu.a = cpu.borrow_wram_mut().save_file_exists() as u8;
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .corrupt
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return try_load_save_data_corrupt(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, BANK(sBackupPlayerData)
    cpu.a = 0x00;
    cpu.pc += 2;
    cpu.cycle(8);

    // call OpenSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fcb); // OpenSRAM
        cpu.pc = pc;
    }

    // ld hl, sBackupPlayerData + wStartDay - wPlayerData
    cpu.set_de(0xb209 + (0xd4b6 - 0xd47b));
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wStartDay
    cpu.set_de(0xd4b6);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, 8
    cpu.set_bc(8);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyBytes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3026); // CopyBytes
        cpu.pc = pc;
    }

    // ld hl, sBackupPlayerData + wStatusFlags - wPlayerData
    cpu.set_de(0xb209 + (0xd84c - 0xd47b));
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wStatusFlags
    cpu.set_de(0xd84c);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // call CloseSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fe1); // CloseSRAM
        cpu.pc = pc;
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn try_load_save_data_corrupt(cpu: &mut Cpu) {
    cpu.pc = 0x4f6c;

    // ld hl, DefaultOptions
    cpu.set_hl(0x4f7c);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wOptions
    cpu.set_de(0xcfcc);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, wOptionsEnd - wOptions
    cpu.set_bc(0xcfd4 - 0xcfcc);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyBytes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3026); // CopyBytes
        cpu.pc = pc;
    }

    // call ClearClock
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x067e); // ClearClock
        cpu.pc = pc;
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
