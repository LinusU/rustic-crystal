use crate::{
    cpu::Cpu,
    game::{
        audio::sfx::Sfx,
        constants::{
            input_constants::JoypadButtons,
            menu_constants::{Menu2DFlags1, NAME_BOX},
            scgb_constants,
        },
        macros::{self, coords::coord},
        ram::{hram, wram},
    },
    save_state::SaveState,
    saves,
};

const MAINMENU_NEW_GAME: u8 = 0;
const MAINMENU_CONTINUE: u8 = 1;

const MAINMENUITEM_CONTINUE: u8 = 0;
const MAINMENUITEM_NEW_GAME: u8 = 1;
const MAINMENUITEM_OPTION: u8 = 2;
const MAINMENUITEM_MYSTERY_GIFT: u8 = 3;
const MAINMENUITEM_MOBILE: u8 = 4;
const MAINMENUITEM_MOBILE_STUDIUM: u8 = 5;

pub fn main_menu(cpu: &mut Cpu) {
    log::debug!("main_menu()");

    loop {
        cpu.borrow_wram_mut().set_disable_text_acceleration(false);

        clear_tilemap_etc(cpu);

        cpu.b = scgb_constants::SCGB_DIPLOMA;
        cpu.call(0x3340); // GetSGBLayout

        cpu.call(0x32f9); // SetDefaultBGPAndOBP

        cpu.borrow_wram_mut().set_game_timer_paused(false);

        {
            let value = main_menu_get_which_menu();
            cpu.borrow_wram_mut().set_which_index_set(value);
        }

        main_menu_print_current_time_and_day(cpu);

        cpu.set_hl(0x5d14); // MainMenu.MenuHeader
        cpu.call(0x1d35); // LoadMenuHeader

        let should_exit_menu = main_menu_joypad_loop(cpu);

        cpu.call(0x1c17); // CloseWindow

        if should_exit_menu {
            cpu.pc = cpu.stack_pop(); // ret
            return;
        }

        cpu.call(0x0fc8); // ClearTilemap

        match cpu.borrow_wram().menu_selection() {
            MAINMENUITEM_CONTINUE => {
                main_menu_select_save(cpu);
            }
            MAINMENUITEM_NEW_GAME => {
                main_menu_create_save(cpu);
            }
            MAINMENUITEM_OPTION => {
                cpu.call(0x5ee7); // MainMenu_Option
            }
            MAINMENUITEM_MYSTERY_GIFT => {
                cpu.call(0x5ef5); // MainMenu_MysteryGift
            }
            MAINMENUITEM_MOBILE => {
                cpu.call(0x5efc); // MainMenu_Mobile
            }
            MAINMENUITEM_MOBILE_STUDIUM => {
                cpu.call(0x6496); // MainMenu_MobileStudium
            }
            n => panic!("Unknown main menu item: {}", n),
        }
    }
}

fn main_menu_get_which_menu() -> u8 {
    match saves::list_save_files() {
        Ok(files) => {
            if files.is_empty() {
                MAINMENU_NEW_GAME
            } else {
                MAINMENU_CONTINUE
            }
        }
        Err(e) => {
            log::error!("Error listing save files: {}", e);
            MAINMENU_NEW_GAME
        }
    }
}

fn main_menu_joypad_loop(cpu: &mut Cpu) -> bool {
    cpu.call(0x1e70); // SetUpMenu

    loop {
        main_menu_print_current_time_and_day(cpu);

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

fn main_menu_print_current_time_and_day(cpu: &mut Cpu) {
    if !cpu.borrow_wram().save_file_exists() {
        return;
    }

    cpu.write_byte(hram::BG_MAP_MODE, 0);

    cpu.call(0x5e27); // MainMenu_PrintCurrentTimeAndDay.PlaceBox

    {
        let saved_options = cpu.read_byte(wram::OPTIONS);
        cpu.borrow_wram_mut().set_no_text_scroll(true);
        cpu.call(0x5e3d); // MainMenu_PrintCurrentTimeAndDay.PlaceTime
        cpu.write_byte(wram::OPTIONS, saved_options);
    }

    cpu.write_byte(hram::BG_MAP_MODE, 1);
}

fn clear_tilemap_etc(cpu: &mut Cpu) {
    cpu.write_byte(hram::MAP_ANIMS, 0);
    cpu.call(0x0fc8); // ClearTilemap
    cpu.call(0x0e5f); // LoadFontsExtra
    cpu.call(0x0e51); // LoadStandardFont
    cpu.call(0x1fbf); // ClearWindowData
}

fn main_menu_select_save(cpu: &mut Cpu) {
    let list = match saves::list_save_files() {
        Ok(ref files) if files.is_empty() => {
            return;
        }
        Ok(files) => files,
        Err(error) => {
            log::error!("Error listing save files: {}", error);
            return;
        }
    };

    let height = u8::min(list.len() as u8 * 2, 16);

    let width = u8::min(
        list.iter()
            .map(|s| s.name.chars().count() as u8)
            .max()
            .unwrap_or(0)
            + 2,
        18,
    );

    let max_menu_item = list.len() - 1;

    let mut selected = 0;
    let mut scroll_pos = 0;

    loop {
        cpu.set_hl(coord!(0, 0));
        cpu.b = height;
        cpu.c = width;
        cpu.call(0x0fe8); // Textbox

        for (i, save_file) in list.iter().skip(scroll_pos).take(8).enumerate() {
            let mut xy = coord!(1, 2 + i as u8 * 2);

            if i + scroll_pos == selected {
                cpu.write_byte(xy, 0xed); // ▶
            } else {
                cpu.write_byte(xy, 0x7f);
            }

            xy += 2;

            for c in save_file.name.chars() {
                let value = match c {
                    'A'..='Z' => 0x80 + (c as u8 - b'A'),
                    'a'..='z' => 0xa0 + (c as u8 - b'a'),
                    '0'..='9' => 0xf6 + (c as u8 - b'0'),
                    ' ' => 0x7f,
                    _ => 0xe6, // ?
                };
                cpu.write_byte(xy, value);
                xy += 1;
            }
        }

        cpu.call(0x0a57); // JoyTextDelay
        cpu.call(0x1bdd); // GetMenuJoypad

        let btns = JoypadButtons::from_bits(cpu.a).unwrap();

        if btns.contains(JoypadButtons::UP) && selected > 0 {
            selected -= 1;

            if selected < scroll_pos {
                scroll_pos = selected;
            }
        }

        if btns.contains(JoypadButtons::DOWN) && selected < max_menu_item {
            selected += 1;

            if selected >= scroll_pos + 8 {
                scroll_pos = selected - 7;
            }
        }

        if btns.contains(JoypadButtons::B) {
            return;
        }

        if btns.contains(JoypadButtons::A) {
            cpu.call(0x2009); // PlayClickSFX

            let save_file = &list[selected];
            let sram = SaveState::from_file(&save_file.path).unwrap();
            cpu.replace_sram(sram, save_file.path.clone());

            macros::farcall::farcall(cpu, 0x05, 0x4f1c); // TryLoadSaveData

            cpu.call(0x5eee); // MainMenu_Continue
            return;
        }

        cpu.call(0x045a); // DelayFrame
    }
}

fn main_menu_create_save(cpu: &mut Cpu) {
    cpu.b = NAME_BOX;
    cpu.set_de(0xd47d); // wPlayerName

    macros::farcall::farcall(cpu, 0x04, 0x56c1); // NamingScreen

    let mut name = String::new();

    for i in 0..8 {
        let c = cpu.read_byte(0xd47d + i);

        match c {
            0x50 => break,                                       // String terminator
            0x80..=0x99 => name.push((c - 0x80 + b'A') as char), // Uppercase letters
            0xa0..=0xb9 => name.push((c - 0xa0 + b'a') as char), // Lowercase letters
            0xf6..=0xff => name.push((c - 0xf6 + b'0') as char), // Digits
            _ => name.push('_'),                                 // Unknown character
        }
    }

    if name.is_empty() || !saves::save_is_free(&name) {
        cpu.play_sfx(Sfx::new(0x3c, 0x497d)); // Sfx_Wrong
        return;
    }

    saves::create_save_dir().unwrap();

    cpu.set_save_path(saves::get_save_path(&name));

    cpu.call(0x5ee0); // MainMenu_NewGame
}
