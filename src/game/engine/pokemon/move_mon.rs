use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{battle_constants::NUM_MOVES, pokemon_constants::PokemonSpecies},
        macros,
    },
    save_state::r#box::BoxedMon,
};

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

    let cur_species = cpu.borrow_wram().cur_party_species();
    cpu.borrow_wram_mut().set_cur_species(cur_species);

    let mon = BoxedMon::from_battle_mon(
        &cpu.borrow_wram().enemy_mon(),
        cpu.borrow_wram().player_id(),
        cpu.borrow_wram().player_name(),
        cur_species.map_or_else(Default::default, |s| s.name()),
    );

    cpu.borrow_sram_mut().current_box_mut().push_front(&mon);

    cpu.a = cur_species.map_or(0, Into::into) - 1;
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
