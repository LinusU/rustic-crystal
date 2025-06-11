use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{input_constants::JoypadButtons, menu_constants::Menu2DFlags1, scgb_constants},
        ram::{hram, sram, wram},
    },
};

const MAINMENU_NEW_GAME: u8 = 0;
const MAINMENU_CONTINUE: u8 = 1;
const MAINMENU_MYSTERY: u8 = 6;

pub fn main_menu(cpu: &mut Cpu) {
    eprintln!("main_menu()");

    loop {
        cpu.borrow_wram_mut().set_disable_text_acceleration(false);

        clear_tilemap_etc(cpu);

        cpu.b = scgb_constants::SCGB_DIPLOMA;
        cpu.call(0x3340); // GetSGBLayout

        cpu.call(0x32f9); // SetDefaultBGPAndOBP

        cpu.borrow_wram_mut().set_game_timer_paused(false);

        {
            let value = main_menu_get_which_menu(cpu);
            eprintln!("main_menu_get_which_menu() -> {}", value);
            cpu.borrow_wram_mut().set_which_index_set(value);
        }

        cpu.call(0x5e09); // MainMenu_PrintCurrentTimeAndDay

        cpu.set_hl(0x5d14); // MainMenu.MenuHeader
        cpu.call(0x1d35); // LoadMenuHeader

        let should_exit_menu = main_menu_joypad_loop(cpu);

        cpu.call(0x1c17); // CloseWindow

        if should_exit_menu {
            cpu.pc = cpu.stack_pop(); // ret
            return;
        }

        cpu.call(0x0fc8); // ClearTilemap

        cpu.a = cpu.borrow_wram().menu_selection();
        cpu.set_hl(0x5d60); // MainMenu.Jumptable
        cpu.call(0x0028); // JumpTable
    }
}

fn main_menu_get_which_menu(cpu: &mut Cpu) -> u8 {
    if !cpu.borrow_wram().save_file_exists() {
        return MAINMENU_NEW_GAME;
    }

    cpu.a = sram::NUM_DAILY_MYSTERY_GIFT_PARTNER_IDS.0;
    cpu.call(0x2fcb); // OpenSRAM

    let num_daily_mystery_gift_partner_ids =
        cpu.read_byte(sram::NUM_DAILY_MYSTERY_GIFT_PARTNER_IDS.1);

    cpu.call(0x2fe1); // CloseSRAM

    if num_daily_mystery_gift_partner_ids != 0xff {
        MAINMENU_MYSTERY
    } else {
        MAINMENU_CONTINUE
    }
}

fn main_menu_joypad_loop(cpu: &mut Cpu) -> bool {
    cpu.call(0x1e70); // SetUpMenu

    loop {
        cpu.call(0x5e09); // MainMenu_PrintCurrentTimeAndDay

        let mut flags = cpu.read_byte(wram::MENU_2D_FLAGS_1);
        flags |= Menu2DFlags1::WRAP_UP_DOWN.bits();
        cpu.write_byte(wram::MENU_2D_FLAGS_1, flags);

        cpu.call(0x1f1a); // GetScrollingMenuJoypad

        let buttons = cpu.borrow_wram().menu_joypad();

        if buttons.contains(JoypadButtons::B) {
            return true;
        }

        if buttons.contains(JoypadButtons::A) {
            cpu.call(0x2009); // PlayClickSFX
            return false;
        }
    }
}

fn clear_tilemap_etc(cpu: &mut Cpu) {
    cpu.write_byte(hram::MAP_ANIMS, 0);
    cpu.call(0x0fc8); // ClearTilemap
    cpu.call(0x0e5f); // LoadFontsExtra
    cpu.call(0x0e51); // LoadStandardFont
    cpu.call(0x1fbf); // ClearWindowData
}
