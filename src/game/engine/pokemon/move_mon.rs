use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::NUM_MOVES,
            pokemon_constants::{PokemonSpecies, EGG},
            ram_constants::{MonType, PokemonWithdrawDepositParameter},
            text_constants::{MON_NAME_LENGTH, NAME_LENGTH},
        },
        macros,
    },
    game_state::{
        box_mon::BoxMonOwned, mon_list::MonListEntry, party_mon::PartyMon, PartyMonSpecies,
    },
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

    let dst_ptr = if action == PokemonWithdrawDepositParameter::PCWithdraw {
        if cpu.borrow_wram().party().is_full() {
            return return_value(cpu, true);
        }

        let species = cpu
            .borrow_wram()
            .cur_party_species()
            .unwrap_or(PokemonSpecies::Unknown(0));

        let party_count = cpu.borrow_wram().party().len();

        cpu.borrow_wram_mut().set_party_count(party_count + 1);
        cpu.borrow_wram_mut()
            .set_party_mon_species(party_count, PartyMonSpecies::Some(species));
        cpu.borrow_wram_mut()
            .set_party_mon_species(party_count + 1, PartyMonSpecies::EndOfListMarker);

        // wPartyMon{N}
        0xdcdf + PartyMon::LEN as u16 * party_count as u16
    } else {
        if cpu.borrow_sram().current_box().is_full() {
            return return_value(cpu, true);
        }

        let species = cpu
            .borrow_wram()
            .cur_party_species()
            .unwrap_or(PokemonSpecies::Unknown(0));

        let list_addr = 0xad10; // sBoxCount
        let list_len = cpu.borrow_sram().current_box().len();

        cpu.write_byte(list_addr, list_len as u8 + 1);
        cpu.set_hl(list_addr + list_len as u16 + 1);

        cpu.write_byte(cpu.hl(), species.into());
        cpu.write_byte(cpu.hl() + 1, 0xff);

        // sBoxMon{N}
        0xad26 + BoxMonOwned::LEN as u16 * list_len as u16
    };

    let src_ptr = if action == PokemonWithdrawDepositParameter::PCWithdraw {
        let idx = cpu.borrow_wram().cur_party_mon();
        0xad26 + BoxMonOwned::LEN as u16 * idx as u16 // sBoxMon{N}
    } else {
        let idx = cpu.borrow_wram().cur_party_mon();
        0xdcdf + PartyMon::LEN as u16 * idx as u16 // wPartyMon{N}
    };

    for i in 0..BoxMonOwned::LEN {
        let val = cpu.read_byte(src_ptr + i as u16);
        cpu.write_byte(dst_ptr + i as u16, val);
    }

    let ot_dst_ptr = if action == PokemonWithdrawDepositParameter::PCDeposit {
        let idx = cpu.borrow_sram().current_box().len() - 1;
        0xafa6 + NAME_LENGTH as u16 * idx as u16 // sBoxMon{N}OT
    } else {
        let idx = cpu.borrow_wram().party().len() - 1;
        0xddff + NAME_LENGTH as u16 * idx as u16 // wPartyMon{N}OT
    };

    let ot_src_ptr = if action == PokemonWithdrawDepositParameter::PCWithdraw {
        let idx = cpu.borrow_wram().cur_party_mon();
        0xafa6 + NAME_LENGTH as u16 * idx as u16 // sBoxMon{N}OT
    } else {
        let idx = cpu.borrow_wram().cur_party_mon();
        0xddff + NAME_LENGTH as u16 * idx as u16 // wPartyMon{N}OT
    };

    for i in 0..NAME_LENGTH {
        let val = cpu.read_byte(ot_src_ptr + i as u16);
        cpu.write_byte(ot_dst_ptr + i as u16, val);
    }

    let nick_dst_ptr = if action == PokemonWithdrawDepositParameter::PCDeposit {
        let idx = cpu.borrow_sram().current_box().len() - 1;
        0xb082 + NAME_LENGTH as u16 * idx as u16 // sBoxMon{N}Nickname
    } else {
        let idx = cpu.borrow_wram().party().len() - 1;
        0xde41 + NAME_LENGTH as u16 * idx as u16 // wPartyMon{N}Nickname
    };

    let nick_src_ptr = if action == PokemonWithdrawDepositParameter::PCWithdraw {
        let idx = cpu.borrow_wram().cur_party_mon();
        0xb082 + NAME_LENGTH as u16 * idx as u16 // sBoxMon{N}Nickname
    } else {
        let idx = cpu.borrow_wram().cur_party_mon();
        0xde41 + NAME_LENGTH as u16 * idx as u16 // wPartyMon{N}Nickname
    };

    for i in 0..MON_NAME_LENGTH {
        let val = cpu.read_byte(nick_src_ptr + i as u16);
        cpu.write_byte(nick_dst_ptr + i as u16, val);
    }

    if action == PokemonWithdrawDepositParameter::PCDeposit {
        cpu.b = (cpu.borrow_sram().current_box().len() as u8) - 1;
        cpu.call(0x5cb6); // RestorePPOfDepositedPokemon
        return return_value(cpu, false);
    }

    cpu.borrow_wram_mut().set_mon_type(MonType::Box);
    macros::predef::predef_call!(cpu, CopyMonToTempMon);
    macros::farcall::callfar(cpu, 0x14, 0x4e1b); // CalcLevel
    let level = cpu.d;

    cpu.borrow_wram_mut().set_cur_party_level(level);
    cpu.write_byte(dst_ptr + 31, level); // MON_LEVEL

    cpu.b = 1; // TRUE
    cpu.set_hl(dst_ptr + 11 - 1); // MON_STAT_EXP - 1
    cpu.set_de(dst_ptr + 36); // MON_MAXHP
    cpu.call(0x6167); // CalcMonStats

    cpu.write_byte(dst_ptr + 32, 0); // MON_STATUS

    if cpu.borrow_wram().cur_party_species() == Some(PokemonSpecies::Unknown(EGG)) {
        cpu.write_byte(dst_ptr + 34, 0); // MON_HP
        cpu.write_byte(dst_ptr + 35, 0); // MON_HP + 1
    } else {
        cpu.a = cpu.read_byte(dst_ptr + 36); // MON_MAXHP
        cpu.write_byte(dst_ptr + 34, cpu.a); // MON_HP
        cpu.a = cpu.read_byte(dst_ptr + 36 + 1); // MON_MAXHP + 1
        cpu.write_byte(dst_ptr + 34 + 1, cpu.a); // MON_HP + 1
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
