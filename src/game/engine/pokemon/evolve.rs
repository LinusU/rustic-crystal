use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::NUM_MOVES,
            item_constants::Item,
            move_constants::Move,
            pokemon_constants::PokemonSpecies,
            pokemon_data_constants::{
                Evolution, HAPPINESS_TO_EVOLVE, MON_MOVES, MON_PP, PARTY_LENGTH,
            },
            ram_constants::MonType,
            serial_constants::LinkMode,
        },
        data::pokemon::evos_attacks::EVOS_ATTACKS,
        macros,
        ram::hram,
    },
    game_state::{mon_list::MonListEntry, PartyMonSpecies},
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

pub fn evolve_after_battle(cpu: &mut Cpu) {
    cpu.borrow_wram_mut().set_mon_tried_to_evolve(false);

    let saved_hl = cpu.hl();
    let saved_bc = cpu.bc();
    let saved_de = cpu.de();

    for party_mon_idx in 0..PARTY_LENGTH {
        cpu.borrow_wram_mut().set_cur_party_mon(party_mon_idx as u8);

        let (species, held_item) = match cpu.borrow_wram().party().get(party_mon_idx) {
            Some(MonListEntry::Mon(mon, ..)) => (mon.species(), mon.item()),
            Some(MonListEntry::Egg(..)) => continue,
            None => break,
        };

        cpu.borrow_wram_mut()
            .set_evolution_old_species(Some(species));

        if cpu.borrow_wram().evolvable_flags() & (1 << party_mon_idx) == 0 {
            continue;
        }

        cpu.borrow_wram_mut().set_mon_type(MonType::Party);
        macros::predef::predef_call!(cpu, CopyMonToTempMon);

        let evos = EVOS_ATTACKS[u8::from(species) as usize - 1].evos;

        for evolution in evos {
            let species_to_evolve_to = match evolution {
                Evolution::Trade(trade_item, species_to_evolve_to) => {
                    if cpu.borrow_wram().link_mode() == LinkMode::Null
                        || held_item == Some(Item::Everstone)
                    {
                        continue;
                    }

                    if let Some(trade_item) = trade_item {
                        if cpu.borrow_wram().link_mode() == LinkMode::TimeCapsule {
                            continue;
                        }

                        if cpu.borrow_wram().temp_mon().item() != Some(*trade_item) {
                            continue;
                        }

                        cpu.borrow_wram_mut().temp_mon_mut().set_item(None);
                    }

                    *species_to_evolve_to
                }

                Evolution::Item(item, species_to_evolve_to) => {
                    if cpu.borrow_wram().link_mode() != LinkMode::Null
                        || cpu.borrow_wram().cur_item() != *item
                        || !cpu.borrow_wram().force_evolution()
                    {
                        continue;
                    }

                    *species_to_evolve_to
                }

                Evolution::LevelUp(level_requirement, species_to_evolve_to) => {
                    if cpu.borrow_wram().link_mode() != LinkMode::Null
                        || cpu.borrow_wram().force_evolution()
                        || cpu.borrow_wram().temp_mon().level() < *level_requirement
                        || held_item == Some(Item::Everstone)
                    {
                        continue;
                    }

                    *species_to_evolve_to
                }

                Evolution::Happiness(trigger, species_to_evolve_to) => {
                    if cpu.borrow_wram().link_mode() != LinkMode::Null
                        || cpu.borrow_wram().force_evolution()
                        || cpu.borrow_wram().temp_mon().happiness() < HAPPINESS_TO_EVOLVE
                        || held_item == Some(Item::Everstone)
                        || !trigger.can_trigger(cpu.borrow_wram().time_of_day())
                    {
                        continue;
                    }

                    *species_to_evolve_to
                }

                Evolution::Stat(evo_level, trigger, species_to_evolve_to) => {
                    let attack = cpu.borrow_wram().temp_mon().attack();
                    let defense = cpu.borrow_wram().temp_mon().defense();

                    if cpu.borrow_wram().link_mode() != LinkMode::Null
                        || cpu.borrow_wram().force_evolution()
                        || cpu.borrow_wram().temp_mon().level() < *evo_level
                        || held_item == Some(Item::Everstone)
                        || !trigger.can_trigger(attack, defense)
                    {
                        continue;
                    }

                    *species_to_evolve_to
                }
            };

            let level = cpu.borrow_wram().temp_mon().level();
            cpu.borrow_wram_mut().set_cur_party_level(level);
            cpu.borrow_wram_mut().set_mon_tried_to_evolve(true);

            cpu.borrow_wram_mut()
                .set_evolution_new_species(Some(species_to_evolve_to));

            cpu.a = cpu.borrow_wram().cur_party_mon();
            cpu.set_hl(0xde41); // wPartyMonNicknames
            cpu.call(0x38a2); // GetNickname

            cpu.call(0x30d6); // CopyName1

            cpu.set_hl(0x6482); // EvolvingText
            cpu.call(0x1057); // PrintText

            cpu.c = 50;
            cpu.call(0x0468); // DelayFrames

            cpu.write_byte(hram::BG_MAP_MODE, 0);

            cpu.b = 12;
            cpu.c = 20;
            cpu.set_hl(macros::coords::coord!(0, 0));
            cpu.call(0x0fb6); // ClearBox

            cpu.write_byte(hram::BG_MAP_MODE, 1);

            cpu.call(0x300b); // ClearSprites

            macros::farcall::farcall(cpu, 0x13, 0x65e1); // EvolutionAnimation
            let user_canceled = cpu.flag(CpuFlag::C);

            cpu.call(0x300b); // ClearSprites

            if user_canceled {
                cpu.set_hl(0x647d); // StoppedEvolvingText
                cpu.call(0x1057); // PrintText
                cpu.call(0x0fc8); // ClearTilemap
                break;
            }

            cpu.set_hl(0x6473); // CongratulationsYourPokemonText
            cpu.call(0x1057); // PrintText

            cpu.borrow_wram_mut()
                .set_cur_species(Some(species_to_evolve_to));

            cpu.borrow_wram_mut()
                .temp_mon_mut()
                .set_species(species_to_evolve_to);

            cpu.borrow_wram_mut()
                .set_evolution_new_species(Some(species_to_evolve_to));

            cpu.borrow_wram_mut()
                .set_named_object_index(species_to_evolve_to.into());

            cpu.call(0x343b); // GetPokemonName

            cpu.set_hl(0x6478); // EvolvedIntoText
            cpu.call(0x1065); // PrintTextboxText

            macros::farcall::farcall(cpu, 0x41, 0x6094); // StubbedTrainerRankings_MonsEvolved

            cpu.set_de(0); // MUSIC_NONE
            cpu.call(0x3b97); // PlayMusic

            cpu.set_de(2); // SFX_CAUGHT_MON
            cpu.call(0x3c23); // PlaySFX

            cpu.call(0x3c55); // WaitSFX

            cpu.c = 40;
            cpu.call(0x0468); // DelayFrames

            cpu.call(0x0fc8); // ClearTilemap

            cpu.call(0x6414); // UpdateSpeciesNameIfNotNicknamed

            cpu.call(0x3856); // GetBaseData

            cpu.set_hl(0xd116 + 2); // wTempMonExp + 2
            cpu.set_de(0xd132); // wTempMonMaxHP
            cpu.b = 1; // TRUE
            macros::predef::predef_call!(cpu, CalcMonStats);

            let party_mon_max_hp = cpu
                .borrow_wram()
                .party()
                .get(party_mon_idx)
                .unwrap()
                .mon()
                .max_hp();

            let temp_mon_hp = cpu.borrow_wram().temp_mon().hp();
            let temp_mon_max_hp = cpu.borrow_wram().temp_mon().max_hp();

            let new_hp =
                temp_mon_hp.saturating_add(temp_mon_max_hp.saturating_sub(party_mon_max_hp));
            cpu.borrow_wram_mut().temp_mon_mut().set_hp(new_hp);

            let temp_mon_copy = cpu.borrow_wram().temp_mon().to_vec();

            cpu.borrow_wram_mut()
                .party_mon_mut(party_mon_idx)
                .copy_from_slice(&temp_mon_copy);

            let species = cpu.borrow_wram().cur_species();
            cpu.borrow_wram_mut().set_temp_species(species);

            cpu.borrow_wram_mut().set_mon_type(MonType::Party);
            cpu.call(0x6487); // LearnLevelMoves

            cpu.a = cpu.borrow_wram().temp_species().map_or(0, Into::into);
            cpu.a = cpu.a.wrapping_sub(1);
            cpu.call(0x3380); // SetSeenAndCaughtMon

            if cpu.borrow_wram().temp_species() == Some(PokemonSpecies::Unown) {
                cpu.set_hl(0xd123); // wTempMonDVs
                macros::predef::predef_call!(cpu, GetUnownLetter);
                macros::farcall::callfar(cpu, 0x3e, 0x7a18); // UpdateUnownDex
            }

            let species = cpu.borrow_wram().temp_mon().species();
            cpu.borrow_wram_mut()
                .set_party_mon_species(party_mon_idx, PartyMonSpecies::Some(species));

            break;
        }
    }

    cpu.set_de(saved_de);
    cpu.set_bc(saved_bc);
    cpu.set_hl(saved_hl);

    if cpu.borrow_wram().link_mode() == LinkMode::Null
        && cpu.borrow_wram().battle_mode().is_none()
        && cpu.borrow_wram().mon_tried_to_evolve()
    {
        cpu.call(0x3d47); // RestartMapMusic
    }

    cpu.pc = cpu.stack_pop(); // ret
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
        let cur_party_mon_moves = cpu.borrow_wram().party().get(idx).unwrap().mon().moves();

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
