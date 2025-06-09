use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::misc_constants,
        data::default_options::DEFAULT_OPTIONS,
        ram::{sram, wram},
    },
};

pub fn try_load_save_data(cpu: &mut Cpu) {
    eprintln!("try_load_save_data()");

    cpu.borrow_wram_mut().set_save_file_exists(false);
    check_primary_save_file(cpu);

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

    for (i, byte) in DEFAULT_OPTIONS.iter().enumerate() {
        cpu.write_byte(0xcfcc + i as u16, *byte); // wOptions + i
    }

    cpu.call(0x067e); // ClearClock

    cpu.pc = cpu.stack_pop(); // ret
}

fn check_primary_save_file(cpu: &mut Cpu) {
    cpu.a = sram::CHECK_VALUE_BANK;
    cpu.call(0x2fcb); // OpenSRAM

    if cpu.read_byte(sram::CHECK_VALUE_1) == misc_constants::SAVE_CHECK_VALUE_1
        && cpu.read_byte(sram::CHECK_VALUE_2) == misc_constants::SAVE_CHECK_VALUE_2
    {
        for i in 0..DEFAULT_OPTIONS.len() as u16 {
            let byte = cpu.read_byte(sram::OPTIONS + i);
            cpu.write_byte(wram::OPTIONS + i, byte);
        }

        cpu.borrow_wram_mut().set_save_file_exists(true);
    }

    cpu.call(0x2fe1); // CloseSRAM
}
