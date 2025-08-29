use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::NUM_MOVES, pokemon_constants::PokemonSpecies,
            ram_constants::PokemonWithdrawDepositParameter,
        },
        macros,
    },
    game_state::{box_mon::BoxMonOwned, mon_list::MonListEntry, party_mon::PartyMonOwned},
};

/// Sents/Gets mon into/from Box depending on Parameter
pub fn send_get_mon_into_from_box(cpu: &mut Cpu) {
    let action = cpu.borrow_wram().pokemon_withdraw_deposit_parameter();

    log::info!("send_get_mon_into_from_box({action:?})");

    fn return_value(cpu: &mut Cpu, value: bool) {
        cpu.call(0x2fe1); // CloseSRAM
        cpu.set_flag(CpuFlag::C, value);
        cpu.pc = cpu.stack_pop(); // ret
    }

    cpu.a = 0x01; // BANK(sBoxCount)
    cpu.call(0x2fcb); // OpenSRAM

    let idx = cpu.borrow_wram().cur_party_mon() as usize;

    if action == PokemonWithdrawDepositParameter::PCDeposit {
        if cpu.borrow_sram().current_box().is_full() {
            return return_value(cpu, true);
        }

        match cpu.borrow_wram().party().get(idx) {
            None => {
                log::error!("send_get_mon_into_from_box called with invalid cur_party_mon {idx}");
                return return_value(cpu, true);
            }

            Some(MonListEntry::Egg(mon, ot_name, nickname)) => {
                let box_mon = BoxMonOwned::from_party_mon(mon);
                let ptr = MonListEntry::Egg(box_mon.as_ref(), ot_name, nickname);
                cpu.borrow_sram_mut().current_box_mut().push_back(ptr);
            }

            Some(MonListEntry::Mon(mon, ot_name, nickname)) => {
                let box_mon = BoxMonOwned::from_party_mon(mon);
                let ptr = MonListEntry::Mon(box_mon.as_ref(), ot_name, nickname);
                cpu.borrow_sram_mut().current_box_mut().push_back(ptr);
            }
        }

        cpu.b = (cpu.borrow_sram().current_box().len() as u8) - 1;
        cpu.call(0x5cb6); // RestorePPOfDepositedPokemon
    } else {
        if cpu.borrow_wram().party().is_full() {
            return return_value(cpu, true);
        }

        match cpu.borrow_sram().current_box().get(idx) {
            None => {
                log::error!("send_get_mon_into_from_box called with invalid cur_party_mon {idx}");
                return return_value(cpu, true);
            }

            Some(MonListEntry::Egg(mon, ot_name, nickname)) => {
                let party_mon = PartyMonOwned::from_box_mon(mon, true);
                cpu.borrow_wram_mut().set_cur_party_level(party_mon.level());
                let ptr = MonListEntry::Egg(party_mon.as_ref(), ot_name, nickname);
                cpu.borrow_wram_mut().party_mut().push_back(ptr);
            }

            Some(MonListEntry::Mon(mon, ot_name, nickname)) => {
                let party_mon = PartyMonOwned::from_box_mon(mon, false);
                cpu.borrow_wram_mut().set_cur_party_level(party_mon.level());
                let ptr = MonListEntry::Mon(party_mon.as_ref(), ot_name, nickname);
                cpu.borrow_wram_mut().party_mut().push_back(ptr);
            }
        }
    }

    return_value(cpu, false)
}

/// Sends the mon into one of Bills Boxes \
/// the data comes mainly from 'wEnemyMon:'
pub fn send_mon_into_box(cpu: &mut Cpu) {
    log::info!(
        "send_mon_into_box({:?})",
        cpu.borrow_wram().cur_party_species(),
    );

    if cpu.borrow_sram().current_box().is_full() {
        cpu.set_flag(CpuFlag::C, false); // return failure
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    cpu.a = 0x01; // BANK(sBoxCount)
    cpu.call(0x2fcb); // OpenSRAM

    let cur_species = cpu
        .borrow_wram()
        .cur_party_species()
        .expect("send_mon_into_box called without cur_party_species");

    cpu.borrow_wram_mut().set_cur_species(Some(cur_species));

    let mon =
        BoxMonOwned::from_battle_mon(cpu.borrow_wram().enemy_mon(), cpu.borrow_wram().player_id());

    let ot_name = cpu.borrow_wram().player_name();

    cpu.borrow_sram_mut()
        .current_box_mut()
        .push_front(MonListEntry::Mon(mon.as_ref(), ot_name, cur_species.name()));

    cpu.a = u8::from(cur_species) - 1;
    cpu.call(0x3380); // SetSeenAndCaughtMon

    if cpu.borrow_wram().cur_party_species() == Some(PokemonSpecies::Unown) {
        cpu.set_hl(0xad3b); // sBoxMon1DVs
        macros::predef::predef_call!(cpu, GetUnownLetter);
        macros::farcall::callfar(cpu, 0x3e, 0x7a18); // UpdateUnownDex
    }

    cpu.set_hl(0xad28); // sBoxMon1Moves
    cpu.set_de(0xd110); // wTempMonMoves
    cpu.set_bc(NUM_MOVES as u16);
    cpu.call(0x3026); // CopyBytes

    cpu.set_hl(0xad3d); // sBoxMon1PP
    cpu.set_de(0xd125); // wTempMonPP
    cpu.set_bc(NUM_MOVES as u16);
    cpu.call(0x3026); // CopyBytes

    cpu.b = 0;
    cpu.call(0x5cb6); // RestorePPOfDepositedPokemon

    cpu.call(0x2fe1); // CloseSRAM

    cpu.set_flag(CpuFlag::C, true); // return success
    cpu.pc = cpu.stack_pop(); // ret
}
