use crate::{
    cpu::{Cpu, CpuFlag},
    game::{constants::scgb_constants, ram::hram},
};

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
            cpu.call(0x5da4); // MainMenu_GetWhichMenu
            let value = cpu.a;
            cpu.borrow_wram_mut().set_which_index_set(value);
        }

        cpu.call(0x5e09); // MainMenu_PrintCurrentTimeAndDay

        cpu.set_hl(0x5d14); // MainMenu.MenuHeader
        cpu.call(0x1d35); // LoadMenuHeader

        cpu.call(0x5de4); // MainMenuJoypadLoop

        cpu.call(0x1c17); // CloseWindow

        if cpu.flag(CpuFlag::C) {
            cpu.pc = cpu.stack_pop(); // ret
            return;
        }

        cpu.call(0x0fc8); // ClearTilemap

        cpu.a = cpu.borrow_wram().menu_selection();
        cpu.set_hl(0x5d60); // MainMenu.Jumptable
        cpu.call(0x0028); // JumpTable
    }
}

fn clear_tilemap_etc(cpu: &mut Cpu) {
    cpu.write_byte(hram::MAP_ANIMS, 0);
    cpu.call(0x0fc8); // ClearTilemap
    cpu.call(0x0e5f); // LoadFontsExtra
    cpu.call(0x0e51); // LoadStandardFont
    cpu.call(0x1fbf); // ClearWindowData
}
