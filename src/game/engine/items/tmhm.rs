use crate::{
    cpu::Cpu,
    game::{
        constants::{
            battle_constants::BattleType,
            gfx_constants::SCREEN_WIDTH,
            item_constants::{NUM_HMS, NUM_TMS},
            text_constants::PrintNum,
        },
        data::moves::tmhm_moves::tmhm_moves,
        macros,
    },
    save_state::string::PokeString,
};

pub fn tm_hm_display_pocket_items(cpu: &mut Cpu) {
    if cpu.borrow_wram().battle_type() == BattleType::Tutorial {
        return cpu.jump(0x4aca); // Tutorial_TMHMPocket
    }

    cpu.set_hl(macros::coords::coord!(5, 2));
    cpu.b = 10;
    cpu.c = 15;
    cpu.a = 0x7f; // ' '
    cpu.call(0x0fb6); // ClearBox

    cpu.call(0x4ab5); // TMHM_GetCurrentPocketPosition
    let cur_idx = cpu.c as usize;

    let mut left = 5;

    for idx in cur_idx.. {
        if idx >= (NUM_TMS + NUM_HMS) {
            cpu.d = left;
            cpu.call(0x4a86); // TMHMPocket_GetCurrentLineCoord
            cpu.set_hl(cpu.hl() + 3);

            cpu.set_de(0x4aae); // TMHM_CancelString
            cpu.call(0x1078); // PlaceString

            break;
        }

        let quantity = cpu.borrow_wram().tms_hms()[idx];

        if quantity == 0 {
            continue;
        }

        cpu.d = left;
        cpu.call(0x4a86); // TMHMPocket_GetCurrentLineCoord
        let cur_line_coord = cpu.hl();

        if idx >= NUM_TMS {
            let hm = (idx - NUM_TMS + 1) as u8;

            const _: () = assert!(NUM_HMS < 10);
            cpu.write_byte(cur_line_coord, 0x80 + (b'H' - b'A'));
            cpu.write_byte(cur_line_coord + 1, 0xf6 + hm);
        } else {
            const _: () = assert!(NUM_TMS < 100);
            let str = PokeString::<2>::from_number(idx as u32 + 1, PrintNum::LEADING_ZEROS);
            cpu.write_byte(cur_line_coord, str[0]);
            cpu.write_byte(cur_line_coord + 1, str[1]);
        }

        let r#move = tmhm_moves().nth(idx).unwrap();

        for (i, &chr) in r#move.name().as_ref().iter().enumerate() {
            if chr == 0x50 {
                break;
            }

            cpu.write_byte(cur_line_coord + 3 + i as u16, chr);
        }

        if idx <= NUM_TMS {
            let base = cur_line_coord + 3 + SCREEN_WIDTH as u16 + 9;
            let str = PokeString::<2>::from_number(quantity as u32, PrintNum::empty());

            cpu.write_byte(base, 0xf1); // '×'
            cpu.write_byte(base + 1, str[0]);
            cpu.write_byte(base + 2, str[1]);
        }

        left -= 1;

        if left == 0 {
            break;
        }
    }

    cpu.d = left;

    cpu.pc = cpu.stack_pop(); // ret
}
