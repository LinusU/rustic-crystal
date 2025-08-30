use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::NUM_MOVES,
            item_data_constants::MAIL_STRUCT_LENGTH,
            pokemon_constants::PokemonSpecies,
            pokemon_data_constants::{MONS_PER_BOX, PARTY_LENGTH},
            ram_constants::{MonType, PokemonWithdrawDepositParameter},
            serial_constants::LinkMode,
            text_constants::MON_NAME_LENGTH,
        },
        macros,
        ram::sram,
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

pub fn restore_pp_of_deposited_pokemon(cpu: &mut Cpu) {
    let idx = cpu.b as usize;

    log::info!("restore_pp_of_deposited_pokemon({idx})");

    let (mut pp, moves) = {
        let r#box = cpu.borrow_sram().current_box();
        let mon = r#box.get(idx).unwrap().mon();
        (mon.pp(), mon.moves())
    };

    log::trace!("restore_pp_of_deposited_pokemon({idx}) Moves: {moves:?}");
    log::trace!("restore_pp_of_deposited_pokemon({idx}) PP before: {pp:?}");

    cpu.borrow_wram_mut().temp_mon_mut().set_pp(pp);
    cpu.borrow_wram_mut().temp_mon_mut().set_moves(&moves);

    let saved_menu_cursor_y = cpu.borrow_wram().menu_cursor_y();
    let saved_mon_type = cpu.borrow_wram().mon_type();

    for (i, pp) in pp.iter_mut().enumerate() {
        if moves.get(i).is_none() {
            break;
        }

        cpu.borrow_wram_mut().temp_mon_mut().set_moves(&moves);
        cpu.borrow_wram_mut().set_mon_type(MonType::Box);
        cpu.borrow_wram_mut().set_menu_cursor_y(i as u8);
        macros::farcall::farcall(cpu, 0x03, 0x78ec); // GetMaxPPOfMove

        *pp = (*pp & 0b11000000) + cpu.borrow_wram().temp_pp();
    }

    cpu.borrow_sram_mut()
        .current_box_mut()
        .get_mut(idx)
        .unwrap()
        .set_pp(pp);

    log::trace!("restore_pp_of_deposited_pokemon({idx}) PP after: {pp:?}");

    cpu.borrow_wram_mut().set_mon_type(saved_mon_type);
    cpu.borrow_wram_mut().set_menu_cursor_y(saved_menu_cursor_y);

    cpu.pc = cpu.stack_pop(); // ret
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

const REMOVE_PARTY: PokemonWithdrawDepositParameter = PokemonWithdrawDepositParameter::PCWithdraw;

pub fn remove_mon_from_party_or_box(cpu: &mut Cpu) {
    let action = cpu.borrow_wram().pokemon_withdraw_deposit_parameter();
    let idx = cpu.borrow_wram().cur_party_mon() as usize;

    if action == REMOVE_PARTY {
        cpu.a = (cpu.borrow_wram().party().len() as u8) - 1;
        cpu.set_hl(0xdcd7); // wPartyCount
    } else {
        cpu.a = 0x01; // BANK(sBoxCount)
        cpu.call(0x2fcb); // OpenSRAM
        cpu.a = (cpu.borrow_sram().current_box().len() as u8) - 1;
        cpu.set_hl(0xad10); // sBoxCount
    }

    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() + 1);

    cpu.set_bc(idx as u16);
    cpu.set_hl(cpu.hl() + idx as u16);
    cpu.set_de(cpu.hl() + idx as u16 + 1);

    loop {
        cpu.a = cpu.read_byte(cpu.de());
        cpu.set_de(cpu.de() + 1);
        cpu.write_byte(cpu.hl(), cpu.a);
        cpu.set_hl(cpu.hl() + 1);

        if cpu.a == 0xff {
            break;
        }
    }

    if action == REMOVE_PARTY {
        cpu.set_hl(0xddff); // wPartyMonOTs
        cpu.d = PARTY_LENGTH as u8 - 1;
    } else {
        cpu.set_hl(0xafa6); // sBoxMonOTs
        cpu.d = MONS_PER_BOX as u8 - 1;
    }

    // If this is the last mon in our party (box),
    // shift all the other mons up to close the gap.
    cpu.a = cpu.borrow_wram().cur_party_mon();
    cpu.call(0x30f4); // SkipNames

    cpu.a = cpu.borrow_wram().cur_party_mon();

    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.d);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.d & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.d);

    // jr nz, .delete_inside
    if cpu.a != cpu.d {
        // Shift the OT names
        cpu.set_de(cpu.hl());
        cpu.set_hl(cpu.hl() + MON_NAME_LENGTH as u16);

        if action == REMOVE_PARTY {
            cpu.set_bc(0xde41); // wPartyMonNicknames
        } else {
            cpu.set_bc(0xb082); // sBoxMonNicknames
        }

        cpu.call(0x318c); // CopyDataUntil

        if action == REMOVE_PARTY {
            cpu.set_hl(0xdcdf); // wPartyMons
            cpu.set_bc(PartyMonOwned::LEN as u16);
        } else {
            cpu.set_hl(0xad26); // sBoxMons
            cpu.set_bc(BoxMonOwned::LEN as u16);
        }

        cpu.a = cpu.borrow_wram().cur_party_mon();
        cpu.call(0x30fe); // AddNTimes

        cpu.set_de(cpu.hl());

        if action == REMOVE_PARTY {
            cpu.set_hl(cpu.hl() + PartyMonOwned::LEN as u16);
            cpu.set_bc(0xddff); // wPartyMonOTs
        } else {
            cpu.set_hl(cpu.hl() + BoxMonOwned::LEN as u16);
            cpu.set_bc(0xafa6); // sBoxMonOTs
        }

        cpu.call(0x318c); // CopyDataUntil

        // Shift the nicknames
        if action == REMOVE_PARTY {
            cpu.set_hl(0xde41); // wPartyMonNicknames
        } else {
            cpu.set_hl(0xb082); // sBoxMonNicknames
        }

        cpu.set_bc(MON_NAME_LENGTH as u16);

        cpu.a = cpu.borrow_wram().cur_party_mon();
        cpu.call(0x30fe); // AddNTimes

        cpu.set_de(cpu.hl());
        cpu.set_hl(cpu.hl() + MON_NAME_LENGTH as u16);

        if action == REMOVE_PARTY {
            cpu.set_bc(0xde83); // wPartyMonNicknamesEnd
        } else {
            cpu.set_bc(0xb15e); // sBoxMonNicknamesEnd
        }

        cpu.call(0x318c); // CopyDataUntil
    } else {
        cpu.write_byte(cpu.hl(), 0xff);
    }

    if action != REMOVE_PARTY {
        return cpu.jump(0x2fe1); // CloseSRAM
    }

    if cpu.borrow_wram().link_mode() != LinkMode::Null {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    // Shift mail
    cpu.a = sram::PARTY_MAIL.0;
    cpu.call(0x2fcb); // OpenSRAM

    // If this is the last mon in our party, no need to shift mail.
    if idx < cpu.borrow_wram().party().len() {
        // Shift our mail messages up.
        cpu.set_bc(MAIL_STRUCT_LENGTH as u16);
        cpu.set_hl(sram::PARTY_MAIL.1 + (idx * MAIL_STRUCT_LENGTH) as u16);

        let saved = cpu.hl();
        cpu.set_hl(cpu.hl() + MAIL_STRUCT_LENGTH as u16);
        cpu.set_de(saved);

        cpu.a = cpu.borrow_wram().cur_party_mon();
        cpu.b = cpu.borrow_wram().cur_party_mon();

        loop {
            let saved_bc = cpu.bc();
            let saved_hl = cpu.hl();

            cpu.set_bc(MAIL_STRUCT_LENGTH as u16);
            cpu.call(0x3026); // CopyBytes

            cpu.set_bc(MAIL_STRUCT_LENGTH as u16);
            cpu.set_hl(saved_hl + MAIL_STRUCT_LENGTH as u16);

            cpu.set_de(saved_hl);
            cpu.set_bc(saved_bc);

            cpu.b = cpu.b.wrapping_add(1);
            cpu.a = cpu.borrow_wram().party().len() as u8;

            if cpu.a == cpu.b {
                break;
            }
        }
    }

    cpu.jump(0x2fe1) // CloseSRAM
}
