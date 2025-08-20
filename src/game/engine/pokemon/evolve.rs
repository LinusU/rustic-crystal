use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::NUM_MOVES,
            move_constants::Move,
            pokemon_constants::PokemonSpecies,
            pokemon_data_constants::{MON_MOVES, MON_PP},
        },
        data::pokemon::evos_attacks::EVOS_ATTACKS,
        macros,
    },
};

impl PokemonSpecies {
    fn pre_evolution(self) -> Option<PokemonSpecies> {
        for (species, data) in PokemonSpecies::iter().zip(EVOS_ATTACKS) {
            for evo in data.evos {
                if evo.species() == self {
                    return Some(species);
                }
            }
        }

        None
    }
}

pub fn learn_level_moves(cpu: &mut Cpu) {
    let level = cpu.borrow_wram().cur_party_level();
    let species = cpu
        .borrow_wram()
        .temp_species()
        .expect("learn_level_moves missing temp_species");

    log::info!("learn_level_moves({level}, {species:?})",);

    cpu.borrow_wram_mut().set_cur_party_species(Some(species));

    let data = &EVOS_ATTACKS[u8::from(species) as usize - 1];

    for &(learn_level, learn_move) in data.level_up {
        if level != learn_level {
            continue;
        }

        let idx = cpu.borrow_wram().cur_party_mon() as usize;
        let cur_party_mon_moves = cpu.borrow_wram().party_mon(idx).moves();

        if cur_party_mon_moves.contains(learn_move) {
            continue;
        }

        cpu.borrow_wram_mut().set_putative_tm_hm_move(learn_move);

        cpu.borrow_wram_mut()
            .set_named_object_index(learn_move.into());

        cpu.d = learn_move.into();
        cpu.call(0x34f8); // GetMoveName
        cpu.call(0x30d6); // CopyName1
        macros::predef::predef_call!(cpu, LearnMove);
    }

    let species = cpu.borrow_wram().cur_party_species();
    cpu.borrow_wram_mut().set_temp_species(species);

    cpu.pc = cpu.stack_pop(); // ret
}

/// Fill in moves at de for wCurPartySpecies at wCurPartyLevel
pub fn fill_moves(cpu: &mut Cpu) {
    let level = cpu.borrow_wram().cur_party_level();
    let species = cpu
        .borrow_wram()
        .cur_party_species()
        .expect("fill_moves missing cur_party_species");

    log::info!("fill_moves({level}, {species:?})");

    let data = &EVOS_ATTACKS[u8::from(species) as usize - 1];
    let level = cpu.borrow_wram().cur_party_level();

    'learn: for &(learn_level, learn_move) in data.level_up {
        if learn_level > level {
            break 'learn;
        }

        if cpu.borrow_wram().skip_moves_before_level_up() != 0 {
            let prev_level = cpu.borrow_wram().prev_party_level();

            if learn_level <= prev_level {
                continue 'learn;
            }
        }

        for c in 0..NUM_MOVES {
            if cpu.read_byte(cpu.de() + c as u16) == learn_move.into() {
                continue 'learn;
            }
        }

        for c in 0..NUM_MOVES {
            if cpu.read_byte(cpu.de() + c as u16) == 0 {
                fill_moves_learn_move(cpu, learn_move, cpu.de() + c as u16);
                continue 'learn;
            }
        }

        shift_moves(cpu, cpu.de());

        if cpu.borrow_wram().evolution_old_species().is_some() {
            shift_moves(cpu, cpu.de() + (MON_PP - MON_MOVES));
        }

        fill_moves_learn_move(cpu, learn_move, cpu.de() + 3);
    }

    cpu.pc = cpu.stack_pop(); // ret
}

fn fill_moves_learn_move(cpu: &mut Cpu, r#move: Move, slot_addr: u16) {
    cpu.write_byte(slot_addr, r#move.into());

    log::debug!("fill_moves_learn_move: {move:?} at {slot_addr:#04x}");

    if cpu.borrow_wram().evolution_old_species().is_some() {
        cpu.write_byte(slot_addr + (MON_PP - MON_MOVES), r#move.pp());
    }
}

fn shift_moves(cpu: &mut Cpu, addr: u16) {
    for c in 0..(NUM_MOVES - 1) {
        let byte = cpu.read_byte(addr + 1 + c as u16);
        cpu.write_byte(addr + c as u16, byte);
    }
}

/// Find the first mon to evolve into `wCurPartySpecies`.
///
/// Return carry and the new species in `wCurPartySpecies`
/// if a pre-evolution is found.
pub fn get_pre_evolution(cpu: &mut Cpu) {
    let input = cpu.borrow_wram().cur_party_species();
    let output = input.and_then(PokemonSpecies::pre_evolution);

    log::info!("get_pre_evolution({input:?}) => {output:?}");

    cpu.set_flag(CpuFlag::C, output.is_some());

    if let Some(species) = output {
        cpu.borrow_wram_mut().set_cur_party_species(Some(species));
    }

    cpu.pc = cpu.stack_pop(); // ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_pre_evolution() {
        assert_eq!(PokemonSpecies::Squirtle.pre_evolution(), None);
        assert_eq!(PokemonSpecies::Wartortle.pre_evolution(), Some(PokemonSpecies::Squirtle));
        assert_eq!(PokemonSpecies::Blastoise.pre_evolution(), Some(PokemonSpecies::Wartortle));

        assert_eq!(PokemonSpecies::Togepi.pre_evolution(), None);

        assert_eq!(PokemonSpecies::Eevee.pre_evolution(), None);
        assert_eq!(PokemonSpecies::Jolteon.pre_evolution(), Some(PokemonSpecies::Eevee));
        assert_eq!(PokemonSpecies::Vaporeon.pre_evolution(), Some(PokemonSpecies::Eevee));
        assert_eq!(PokemonSpecies::Flareon.pre_evolution(), Some(PokemonSpecies::Eevee));
        assert_eq!(PokemonSpecies::Espeon.pre_evolution(), Some(PokemonSpecies::Eevee));
        assert_eq!(PokemonSpecies::Umbreon.pre_evolution(), Some(PokemonSpecies::Eevee));

        assert_eq!(PokemonSpecies::Mewtwo.pre_evolution(), None);
        assert_eq!(PokemonSpecies::Mew.pre_evolution(), None);
    }
}
