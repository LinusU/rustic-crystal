use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{menu_constants::NAME_BOX, text_constants::BOX_NAME_LENGTH},
        macros,
    },
};

pub fn bills_pc_change_box_submenu(cpu: &mut Cpu) {
    log::info!("bills_pc_change_box_submenu()");

    cpu.set_hl(0x777b); // BillsPC_ChangeBoxSubmenu.MenuHeader
    cpu.call(0x1d35); // LoadMenuHeader
    cpu.call(0x1d81); // VerticalMenu
    cpu.call(0x1c07); // ExitMenu

    if !cpu.flag(CpuFlag::C) {
        match cpu.borrow_wram().menu_cursor_y() {
            1 => bills_pc_change_box_submenu_switch(cpu),
            2 => bills_pc_change_box_submenu_name(cpu),
            3 => bills_pc_change_box_submenu_print(cpu),
            _ => {}
        }
    }

    cpu.pc = cpu.stack_pop(); // ret
}

fn bills_pc_change_box_submenu_print(cpu: &mut Cpu) {
    cpu.call(0x766c); // GetBoxCount

    if cpu.a == 0 {
        cpu.call(0x77be); // BillsPC_PlaceEmptyBoxString_SFX
    } else {
        cpu.e = cpu.l;
        cpu.d = cpu.h;

        cpu.a = cpu.borrow_wram().menu_selection() - 1;
        cpu.c = cpu.a;

        macros::farcall::farcall(cpu, 0x21, 0x44bc); // PrintPCBox
        cpu.call(0x75e2); // BillsPC_ClearTilemap
    }
}

fn bills_pc_change_box_submenu_switch(cpu: &mut Cpu) {
    cpu.e = cpu.borrow_wram().menu_selection() - 1;
    cpu.a = cpu.borrow_wram().cur_box();

    if cpu.a != cpu.e {
        macros::farcall::farcall(cpu, 0x05, 0x4a83); // ChangeBoxSaveGame
    }
}

fn bills_pc_change_box_submenu_name(cpu: &mut Cpu) {
    cpu.b = NAME_BOX;
    cpu.set_de(0xd002); // wBoxNameBuffer
    macros::farcall::farcall(cpu, 0x04, 0x56c1); // NamingScreen

    cpu.call(0x0fc8); // ClearTilemap
    cpu.call(0x0e51); // LoadStandardFont
    cpu.call(0x0e58); // LoadFontsBattleExtra

    cpu.a = cpu.borrow_wram().menu_selection() - 1;
    cpu.call(0x7626); // GetBoxName
    let name_ptr = cpu.hl();

    cpu.set_hl(0xd002); // wBoxNameBuffer
    cpu.set_de(name_ptr);
    cpu.c = BOX_NAME_LENGTH as u8 - 1;
    cpu.call(0x2ef6); // InitString

    cpu.a = cpu.borrow_wram().menu_selection() - 1;
    cpu.call(0x7626); // GetBoxName

    cpu.set_de(0xd002); // wBoxNameBuffer
    cpu.call(0x30d9); // CopyName2
}
