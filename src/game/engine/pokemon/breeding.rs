use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{battle_constants::NUM_MOVES, move_constants::Move},
        data::{moves::tmhm_moves::tmhm_moves, pokemon::evos_attacks::EVOS_ATTACKS},
        macros,
    },
};

pub fn get_egg_move(cpu: &mut Cpu) {
    let species = cpu.borrow_wram().egg_mon().unwrap().species();
    let de_move = Move::from(cpu.read_byte(cpu.de()));

    log::info!("get_egg_move({species:?}, {de_move:?})");

    fn return_value(cpu: &mut Cpu, value: bool) {
        cpu.set_flag(CpuFlag::C, value);
        cpu.pc = cpu.stack_pop(); // ret
    }

    if species.egg_moves().contains(&de_move) {
        return return_value(cpu, true);
    }

    let saved_bc = cpu.bc();
    cpu.call(0x720b); // GetBreedmonMovePointer
    let breedmon_ptr = cpu.hl();
    cpu.set_bc(saved_bc);

    for i in 0..NUM_MOVES {
        let breedmon_move = cpu.read_byte(breedmon_ptr + i as u16);

        if breedmon_move == de_move.into() {
            let data = EVOS_ATTACKS[u8::from(species) as usize - 1].level_up;

            if data.iter().any(|&(_, r#move)| de_move == r#move) {
                return return_value(cpu, true);
            }

            break;
        }
    }

    if tmhm_moves().any(|m| m == de_move) {
        cpu.borrow_wram_mut().set_putative_tm_hm_move(de_move);
        let saved_bc = cpu.bc();
        macros::predef::predef_call!(cpu, CanLearnTMHMMove);
        cpu.a = cpu.c;
        cpu.set_bc(saved_bc);

        if cpu.a != 0 {
            return return_value(cpu, true);
        }
    }

    return_value(cpu, false)
}
