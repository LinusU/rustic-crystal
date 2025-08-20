use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{move_constants::Move, pokemon_constants::PokemonSpecies},
        data::{moves::tmhm_moves::tmhm_moves, pokemon::evos_attacks::EVOS_ATTACKS},
        macros,
    },
    game_state::moveset::Moveset,
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

    if get_breedmon_moves(cpu).contains(de_move) {
        let data = EVOS_ATTACKS[u8::from(species) as usize - 1].level_up;

        if data.iter().any(|&(_, r#move)| de_move == r#move) {
            return return_value(cpu, true);
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

fn get_breedmon_moves(cpu: &mut Cpu) -> Moveset {
    if let Some(mon) = cpu.borrow_wram().breed_mon_1() {
        if mon.species() == PokemonSpecies::Ditto {
            return mon.moves();
        }
    }

    if let Some(mon) = cpu.borrow_wram().breed_mon_2() {
        if mon.species() == PokemonSpecies::Ditto {
            return mon.moves();
        }
    }

    let mon = if cpu.borrow_wram().breed_mother_or_non_ditto() {
        cpu.borrow_wram().breed_mon_1()
    } else {
        cpu.borrow_wram().breed_mon_2()
    };

    mon.map_or([0, 0, 0, 0].into(), |mon| mon.moves())
}
