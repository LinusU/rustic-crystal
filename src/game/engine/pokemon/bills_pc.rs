use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{menu_constants::NAME_BOX, text_constants::BOX_NAME_LENGTH},
        macros,
    },
};

pub fn bills_pc_change_box_submenu(cpu: &mut Cpu) {
    log::info!("bills_pc_change_box_submenu()");

    cpu.pc = 0x76f9;

    // ld hl, .MenuHeader
    cpu.set_hl(0x777b); // BillsPC_ChangeBoxSubmenu.MenuHeader
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

    // call VerticalMenu
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1d81); // VerticalMenu
        cpu.pc = pc;
    }

    // call ExitMenu
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1c07); // ExitMenu
        cpu.pc = pc;
    }

    // ret c
    if cpu.flag(CpuFlag::C) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [wMenuCursorY]
    cpu.a = cpu.borrow_wram().menu_cursor_y();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp $1
    cpu.set_flag(CpuFlag::Z, cpu.a == 0x1);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (0x1 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0x1);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .Switch
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return bills_pc_change_box_submenu_switch(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // cp $2
    cpu.set_flag(CpuFlag::Z, cpu.a == 0x2);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (0x2 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0x2);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .Name
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return bills_pc_change_box_submenu_name(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // cp $3
    cpu.set_flag(CpuFlag::Z, cpu.a == 0x3);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (0x3 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0x3);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .Print
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return bills_pc_change_box_submenu_print(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn bills_pc_change_box_submenu_print(cpu: &mut Cpu) {
    cpu.pc = 0x7717;

    // call GetBoxCount
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x766c); // GetBoxCount
        cpu.pc = pc;
    }

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .EmptyBox
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return bills_pc_change_box_submenu_empty_box(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld e, l
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld d, h
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [wMenuSelection]
    cpu.a = cpu.borrow_wram().menu_selection();
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // farcall PrintPCBox
    macros::farcall::farcall(cpu, 0x21, 0x44bc);

    // call BillsPC_ClearTilemap
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x75e2); // BillsPC_ClearTilemap
        cpu.pc = pc;
    }

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn bills_pc_change_box_submenu_empty_box(cpu: &mut Cpu) {
    cpu.pc = 0x772f;

    // call BillsPC_PlaceEmptyBoxString_SFX
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x77be); // BillsPC_PlaceEmptyBoxString_SFX
        cpu.pc = pc;
    }

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn bills_pc_change_box_submenu_switch(cpu: &mut Cpu) {
    cpu.pc = 0x7734;

    // ld a, [wMenuSelection]
    cpu.a = cpu.borrow_wram().menu_selection();
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld e, a
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [wCurBox]
    cpu.a = cpu.borrow_wram().cur_box();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp e
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.e);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.e & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.e);
    cpu.pc += 1;
    cpu.cycle(4);

    // ret z
    if cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // farcall ChangeBoxSaveGame
    macros::farcall::farcall(cpu, 0x05, 0x4a83);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn bills_pc_change_box_submenu_name(cpu: &mut Cpu) {
    cpu.pc = 0x7745;

    // ld b, NAME_BOX
    cpu.b = NAME_BOX;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld de, wBoxNameBuffer
    cpu.set_de(0xd002); // wBoxNameBuffer
    cpu.pc += 3;
    cpu.cycle(12);

    // farcall NamingScreen
    macros::farcall::farcall(cpu, 0x04, 0x56c1);

    // call ClearTilemap
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0fc8); // ClearTilemap
        cpu.pc = pc;
    }

    // call LoadStandardFont
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0e51); // LoadStandardFont
        cpu.pc = pc;
    }

    // call LoadFontsBattleExtra
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0e58); // LoadFontsBattleExtra
        cpu.pc = pc;
    }

    // ld a, [wMenuSelection]
    cpu.a = cpu.borrow_wram().menu_selection();
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // call GetBoxName
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x7626); // GetBoxName
        cpu.pc = pc;
    }

    // ld e, l
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld d, h
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld hl, wBoxNameBuffer
    cpu.set_hl(0xd002); // wBoxNameBuffer
    cpu.pc += 3;
    cpu.cycle(12);

    // ld c, BOX_NAME_LENGTH - 1
    cpu.c = BOX_NAME_LENGTH as u8 - 1;
    cpu.pc += 2;
    cpu.cycle(8);

    // call InitString
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2ef6); // InitString
        cpu.pc = pc;
    }

    // ld a, [wMenuSelection]
    cpu.a = cpu.borrow_wram().menu_selection();
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // call GetBoxName
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x7626); // GetBoxName
        cpu.pc = pc;
    }

    // ld de, wBoxNameBuffer
    cpu.set_de(0xd002); // wBoxNameBuffer
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyName2
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x30d9); // CopyName2
        cpu.pc = pc;
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
