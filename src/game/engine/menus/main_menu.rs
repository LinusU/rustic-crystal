use crate::{
    cpu::{Cpu, CpuFlag},
    game::constants::scgb_constants,
};

pub fn main_menu(cpu: &mut Cpu) {
    cpu.pc = 0x5cdc;

    eprintln!("main_menu()");

    main_menu_loop(cpu);
}

fn main_menu_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5cdc;

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wDisableTextAcceleration], a
    cpu.borrow_wram_mut().set_disable_text_acceleration(false);
    cpu.pc += 3;
    cpu.cycle(16);

    // call ClearTilemapEtc
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x5ed0); // ClearTilemapEtc
        cpu.pc = pc;
    }

    // ld b, SCGB_DIPLOMA
    cpu.b = scgb_constants::SCGB_DIPLOMA;
    cpu.pc += 2;
    cpu.cycle(8);

    // call GetSGBLayout
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3340); // GetSGBLayout
        cpu.pc = pc;
    }

    // call SetDefaultBGPAndOBP
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x32f9); // SetDefaultBGPAndOBP
        cpu.pc = pc;
    }

    // ld hl, wGameTimerPaused
    // res GAME_TIMER_COUNTING_F, [hl]
    cpu.borrow_wram_mut().set_game_timer_paused(false);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.pc += 2;
    cpu.cycle(16);

    // call MainMenu_GetWhichMenu
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x5da4); // MainMenu_GetWhichMenu
        cpu.pc = pc;
    }

    // ld [wWhichIndexSet], a
    {
        let value = cpu.a;
        cpu.borrow_wram_mut().set_which_index_set(value);
        cpu.pc += 3;
        cpu.cycle(16);
    }

    // call MainMenu_PrintCurrentTimeAndDay
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x5e09); // MainMenu_PrintCurrentTimeAndDay
        cpu.pc = pc;
    }

    // ld hl, .MenuHeader
    cpu.set_hl(0x5d14); // MainMenu.MenuHeader
    cpu.pc += 3;
    cpu.cycle(12);

    // call LoadMenuHeader
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1d35); // LoadMenuHeader
        cpu.pc = pc;
    }

    // call MainMenuJoypadLoop
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x5de4); // MainMenuJoypadLoop
        cpu.pc = pc;
    }

    // call CloseWindow
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1c17); // CloseWindow
        cpu.pc = pc;
    }

    // jr c, .quit
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return main_menu_quit(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call ClearTilemap
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0fc8); // ClearTilemap
        cpu.pc = pc;
    }

    // ld a, [wMenuSelection]
    cpu.a = cpu.borrow_wram().menu_selection();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld hl, .Jumptable
    cpu.set_hl(0x5d60); // MainMenu.Jumptable
    cpu.pc += 3;
    cpu.cycle(12);

    // rst JumpTable
    {
        cpu.pc += 1;
        let pc = cpu.pc;
        cpu.cycle(16);
        cpu.call(0x0028); // JumpTable
        cpu.pc = pc;
    }

    // jr .loop
    cpu.cycle(12);
    main_menu_loop(cpu)
}

fn main_menu_quit(cpu: &mut Cpu) {
    cpu.pc = 0x5d13;

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
