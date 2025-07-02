use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_tower_constants::BATTLETOWER_RECEIVED_REWARD, misc_constants,
            pokemon_data_constants::NUM_BOXES,
        },
        data::default_options::DEFAULT_OPTIONS,
        macros,
        ram::{sram, wram},
    },
};

pub fn change_box_save_game(cpu: &mut Cpu) {
    let target_box_idx = cpu.e;
    log::info!("change_box_save_game({target_box_idx})");

    cpu.set_hl(0x52a1); // ChangeBoxSaveText
    cpu.call(0x1d4f); // MenuTextbox
    cpu.call(0x1dcf); // YesNoBox
    cpu.call(0x1c07); // ExitMenu

    if cpu.flag(CpuFlag::C) {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    cpu.call(0x4b89); // AskOverwriteSaveFile

    if cpu.flag(CpuFlag::C) {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    cpu.call(0x4b54); // PauseGameLogic
    cpu.call(0x4c99); // SavingDontTurnOffThePower

    cpu.call(0x4e0c); // SaveBox

    cpu.borrow_wram_mut().set_cur_box(target_box_idx);
    cpu.call(0x5021); // LoadBox

    cpu.call(0x4be6); // SavedTheGame
    cpu.call(0x4b5a); // ResumeGameLogic

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn save_game_data(cpu: &mut Cpu) {
    log::debug!("save_game_data()");

    cpu.borrow_wram_mut().set_save_file_exists(true);

    macros::farcall::farcall(cpu, 0x05, 0x4056); // StageRTCTimeForSave
    macros::farcall::farcall(cpu, 0x41, 0x50d9); // BackupMysteryGift

    cpu.call(0x4da9); // ValidateSave
    cpu.call(0x4dbb); // SaveOptions
    cpu.call(0x4dd7); // SavePlayerData
    cpu.call(0x4df7); // SavePokemonData
    cpu.call(0x4e0c); // SaveBox
    cpu.call(0x4e13); // SaveChecksum
    cpu.call(0x4e2d); // ValidateBackupSave
    cpu.call(0x4e40); // SaveBackupOptions
    cpu.call(0x4e55); // SaveBackupPlayerData
    cpu.call(0x4e76); // SaveBackupPokemonData
    cpu.call(0x4e8b); // SaveBackupChecksum
    cpu.call(0x4c6b); // UpdateStackTop

    macros::farcall::farcall(cpu, 0x11, 0x4725); // BackupPartyMonMail
    macros::farcall::farcall(cpu, 0x41, 0x6187); // BackupGSBallFlag
    macros::farcall::farcall(cpu, 0x05, 0x406a); // SaveRTC

    cpu.a = sram::BATTLE_TOWER_CHALLENGE_STATE.0;
    cpu.call(0x2fcb); // OpenSRAM

    if cpu.read_byte(sram::BATTLE_TOWER_CHALLENGE_STATE.1) != BATTLETOWER_RECEIVED_REWARD {
        cpu.write_byte(sram::BATTLE_TOWER_CHALLENGE_STATE.1, 0);
    }

    cpu.call(0x2fe1); // CloseSRAM

    cpu.save_to_disk();

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn save_box(cpu: &mut Cpu) {
    log::info!("save_box({})", cpu.borrow_wram().cur_box());
    get_box_address(cpu);
    save_box_address(cpu);
    cpu.pc = cpu.stack_pop(); // ret
}

pub fn try_load_save_data(cpu: &mut Cpu) {
    log::debug!("try_load_save_data()");

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

pub fn load_box(cpu: &mut Cpu) {
    log::info!("load_box({})", cpu.borrow_wram().cur_box());
    get_box_address(cpu);
    load_box_address(cpu);
    cpu.pc = cpu.stack_pop(); // ret
}

fn get_box_address(cpu: &mut Cpu) {
    let mut cur_box = cpu.borrow_wram().cur_box();

    if cur_box >= NUM_BOXES {
        log::warn!("cur_box out of bounds: {cur_box}");
        cpu.borrow_wram_mut().set_cur_box(0);
        cur_box = 0;
    }

    const BASE: u16 = 0x522d; // BoxAddresses
    let offset = BASE + (cur_box as u16 * 5);

    cpu.a = cpu.read_byte(offset);
    cpu.e = cpu.read_byte(offset + 1);
    cpu.d = cpu.read_byte(offset + 2);
    cpu.l = cpu.read_byte(offset + 3);
    cpu.h = cpu.read_byte(offset + 4);
}

/// Copies the current box data to the address specified by the A:DE registers.
pub fn save_box_address(cpu: &mut Cpu) {
    let bank = cpu.a as usize;
    let addr = cpu.de() as usize;

    log::info!("save_box_address({bank:02x}:{addr:04x})");

    let offset = (bank * 0x2000) | (addr & 0x1fff);
    let sram = cpu.borrow_sram_mut();

    const BOX_LENGTH: usize = 0x450;
    const SOURCE: usize = 0x2d10; // Current Box

    for i in 0..BOX_LENGTH {
        let byte = sram.byte(SOURCE + i);
        sram.set_byte(offset + i, byte);
    }
}

/// Loads the box data from the address specified by the A:DE registers into the current box.
pub fn load_box_address(cpu: &mut Cpu) {
    let bank = cpu.a as usize;
    let addr = cpu.de() as usize;

    log::info!("load_box_address({bank:02x}:{addr:04x})");

    let offset = (bank * 0x2000) | (addr & 0x1fff);
    let sram = cpu.borrow_sram_mut();

    const BOX_LENGTH: usize = 0x450;
    const DESTINATION: usize = 0x2d10; // Current Box

    for i in 0..BOX_LENGTH {
        let byte = sram.byte(offset + i);
        sram.set_byte(DESTINATION + i, byte);
    }
}
