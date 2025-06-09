use crate::cpu::{Cpu, CpuFlag};

pub fn try_load_save_data(cpu: &mut Cpu) {
    eprintln!("try_load_save_data()");

    cpu.borrow_wram_mut().set_save_file_exists(false);
    cpu.call(0x4f84); // CheckPrimarySaveFile

    if !cpu.borrow_wram_mut().save_file_exists() {
        return try_load_save_data_backup(cpu);
    }

    cpu.a = 1; // BANK(sPlayerData)
    cpu.call(0x2fcb); // OpenSRAM

    cpu.set_hl(0xa009 + (0xd4b6 - 0xd47b)); // sPlayerData + wStartDay - wPlayerData
    cpu.set_de(0xd4b6); // wStartDay
    cpu.set_bc(8);
    cpu.call(0x3026); // CopyBytes

    cpu.set_hl(0xa009 + (0xd84c - 0xd47b)); // sPlayerData + wStatusFlags - wPlayerData
    cpu.set_de(0xd84c); // wStatusFlags
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.write_byte(cpu.de(), cpu.a);

    cpu.call(0x2fe1); // CloseSRAM

    cpu.pc = cpu.stack_pop(); // ret
}

fn try_load_save_data_backup(cpu: &mut Cpu) {
    cpu.call(0x4faf); // CheckBackupSaveFile

    cpu.a = cpu.borrow_wram_mut().save_file_exists() as u8;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);

    if !cpu.borrow_wram_mut().save_file_exists() {
        return try_load_save_data_corrupt(cpu);
    }

    cpu.a = 0; // BANK(sBackupPlayerData)
    cpu.call(0x2fcb); // OpenSRAM

    cpu.set_hl(0xb209 + (0xd4b6 - 0xd47b)); // sBackupPlayerData + wStartDay - wPlayerData
    cpu.set_de(0xd4b6); // wStartDay
    cpu.set_bc(8);
    cpu.call(0x3026); // CopyBytes

    cpu.set_hl(0xb209 + (0xd84c - 0xd47b)); // sBackupPlayerData + wStatusFlags - wPlayerData
    cpu.set_de(0xd84c); // wStatusFlags
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.write_byte(cpu.de(), cpu.a);

    cpu.call(0x2fe1); // CloseSRAM

    cpu.pc = cpu.stack_pop(); // ret
}

fn try_load_save_data_corrupt(cpu: &mut Cpu) {
    cpu.pc = 0x4f6c;

    cpu.set_hl(0x4f7c); // DefaultOptions
    cpu.set_de(0xcfcc); // wOptions
    cpu.set_bc(0xcfd4 - 0xcfcc); // wOptionsEnd - wOptions
    cpu.call(0x3026); // CopyBytes

    cpu.call(0x067e); // ClearClock

    cpu.pc = cpu.stack_pop(); // ret
}
