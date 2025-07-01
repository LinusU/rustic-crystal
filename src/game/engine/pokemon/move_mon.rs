use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::{NUM_EXP_STATS, NUM_MOVES},
            pokemon_constants::PokemonSpecies,
            pokemon_data_constants::BASE_HAPPINESS,
            text_constants::{MON_NAME_LENGTH, NAME_LENGTH},
        },
        macros,
        ram::{hram, sram, wram},
    },
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

    let new_len = cpu.borrow_sram().current_box().len() + 1;
    cpu.write_byte(0xad10, new_len); // sBoxCount

    let cur_species = cpu.borrow_wram().cur_party_species();
    cpu.borrow_wram_mut().set_cur_species(cur_species);

    cpu.c = cur_species.map_or(0, Into::into);
    cpu.set_de(0xad10); // sBoxCount

    loop {
        cpu.set_de(cpu.de().wrapping_add(1));
        cpu.a = cpu.c;
        cpu.c = cpu.read_byte(cpu.de());
        cpu.write_byte(cpu.de(), cpu.a);

        if cpu.a == 0xff {
            break;
        }
    }

    cpu.call(0x3856); // GetBaseData
    cpu.call(0x5f47); // ShiftBoxMon

    cpu.set_hl(0xd47d); // wPlayerName
    cpu.set_de(0xafa6); // sBoxMonOTs
    cpu.set_bc(NAME_LENGTH as u16);
    cpu.call(0x3026); // CopyBytes

    cpu.a = cpu.borrow_wram().cur_party_species().map_or(0, Into::into);
    let named_object_index = cpu.a;
    cpu.borrow_wram_mut()
        .set_named_object_index(named_object_index);

    cpu.call(0x343b); // GetPokemonName

    cpu.set_hl(wram::STRING_BUFFER_1);
    cpu.set_de(sram::BOX_MON_NICKNAMES.1);
    cpu.set_bc(MON_NAME_LENGTH as u16);
    cpu.call(0x3026); // CopyBytes

    cpu.set_hl(0xd206); // wEnemyMon
    cpu.set_de(0xad26); // sBoxMon1
    cpu.set_bc(1 + 1 + NUM_MOVES as u16); // species + item + moves
    cpu.call(0x3026); // CopyBytes

    cpu.set_hl(0xd47b); // wPlayerID
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.set_de(cpu.de().wrapping_add(1));

    let saved_de = cpu.de();
    cpu.d = cpu.borrow_wram().cur_party_level();
    macros::farcall::callfar(cpu, 0x14, 0x4e47); // CalcExpAtLevel
    cpu.set_de(saved_de);

    cpu.a = cpu.read_byte(hram::PRODUCT + 1);
    cpu.write_byte(cpu.de(), cpu.a);

    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.a = cpu.read_byte(hram::PRODUCT + 2);
    cpu.write_byte(cpu.de(), cpu.a);

    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.a = cpu.read_byte(hram::PRODUCT + 3);
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.set_de(cpu.de().wrapping_add(1));

    // Set all 5 Experience Values to 0
    cpu.b = 2 * NUM_EXP_STATS;

    loop {
        cpu.write_byte(cpu.de(), 0);
        cpu.set_de(cpu.de() + 1);
        cpu.b -= 1;

        if cpu.b == 0 {
            break;
        }
    }

    cpu.set_hl(0xd20c); // wEnemyMonDVs
    cpu.b = 2 + NUM_MOVES; // DVs and PP ; wEnemyMonHappiness - wEnemyMonDVs

    loop {
        cpu.a = cpu.read_byte(cpu.hl());
        cpu.set_hl(cpu.hl() + 1);
        cpu.write_byte(cpu.de(), cpu.a);
        cpu.set_de(cpu.de() + 1);

        cpu.b -= 1;

        if cpu.b == 0 {
            break;
        }
    }

    cpu.write_byte(cpu.de(), BASE_HAPPINESS);
    cpu.set_de(cpu.de().wrapping_add(1));

    cpu.write_byte(cpu.de(), 0);
    cpu.set_de(cpu.de() + 1);
    cpu.write_byte(cpu.de(), 0);
    cpu.set_de(cpu.de() + 1);
    cpu.write_byte(cpu.de(), 0);
    cpu.set_de(cpu.de() + 1);

    cpu.a = cpu.borrow_wram().cur_party_level();
    cpu.write_byte(cpu.de(), cpu.a);

    cpu.a = cpu.borrow_wram().cur_party_species().map_or(0, Into::into);
    cpu.a = cpu.a.wrapping_sub(1);
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
