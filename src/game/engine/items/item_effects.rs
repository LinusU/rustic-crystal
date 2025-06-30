use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::{self, BattleResult, BattleType},
            item_constants::{Item, SAFARI_BALL},
            item_data_constants::HELD_CATCH_CHANCE,
            menu_constants, move_constants,
            pokemon_constants::PokemonSpecies,
            pokemon_data_constants::{
                FRIEND_BALL_HAPPINESS, MONS_PER_BOX, PARTYMON_STRUCT_LENGTH, PARTY_LENGTH,
            },
            ram_constants::MonType,
            text_constants,
        },
        macros,
        ram::{hram, sram, wram},
    },
};

// BUG: The Dude's catching tutorial may crash if his Poké Ball can't be used
pub fn poke_ball_effect(cpu: &mut Cpu) {
    log::info!("poke_ball_effect()");

    if cpu.borrow_wram().battle_mode() != Some(battle_constants::BattleMode::Wild) {
        return cpu.jump(0x77a0); // UseBallInTrainerBattle
    }

    if cpu.borrow_wram().party_count() == PARTY_LENGTH
        && cpu.borrow_sram().box_count() == MONS_PER_BOX
    {
        return cpu.jump(0x77dc); // Ball_BoxIsFullMessage
    }

    cpu.borrow_wram_mut().set_wild_mon(None);

    if cpu.borrow_wram().battle_type() != BattleType::Contest {
        macros::farcall::farcall(cpu, 0x09, 0x715c); // _ReturnToBattle_UseBall
    }

    cpu.borrow_wram_mut().set_no_text_scroll(false);

    cpu.set_hl(0x783d); // ItemUsedText
    cpu.call(0x1057); // PrintText

    if cpu.borrow_wram().battle_type() == BattleType::Tutorial {
        return poke_ball_effect_catch(cpu, Some(cpu.borrow_wram().enemy_mon().species()));
    }

    cpu.b = cpu.borrow_wram().enemy_mon_catch_rate();

    log::trace!("base catch rate: {}", cpu.b);
    log::trace!("ball: {:?}", cpu.borrow_wram().cur_item());

    match cpu.borrow_wram().cur_item() {
        Item::MasterBall => {
            return poke_ball_effect_catch(cpu, Some(cpu.borrow_wram().enemy_mon().species()));
        }

        Item::UltraBall => {
            cpu.b = cpu.b.saturating_mul(2);
        }

        Item::GreatBall | Item::ParkBall | SAFARI_BALL => {
            cpu.b = cpu.b.saturating_add(cpu.b / 2);
        }

        Item::HeavyBall => cpu.call(0x6c50), // HeavyBallMultiplier
        Item::LevelBall => cpu.call(0x6d8c), // LevelBallMultiplier
        Item::LureBall => cpu.call(0x6ccc),  // LureBallMultiplier
        Item::FastBall => cpu.call(0x6d68),  // FastBallMultiplier
        Item::MoonBall => cpu.call(0x6cdd),  // MoonBallMultiplier
        Item::LoveBall => cpu.call(0x6d12),  // LoveBallMultiplier

        Item::PokeBall => {} // no special multiplier

        n => log::warn!("Unknown item used as ball: {n:?}"),
    }

    let mut catch_rate = cpu.b;
    log::trace!("catch rate after ball effect: {catch_rate}");

    if cpu.borrow_wram().cur_item() != Item::LevelBall {
        let mut h2 = cpu.borrow_wram().enemy_mon().hp() * 2;
        let mut m3 = cpu.borrow_wram().enemy_mon().max_hp() * 3;

        if m3 > 255 {
            h2 /= 4;
            m3 /= 4;

            if h2 == 0 {
                h2 = 1;
            }
        }

        let h2 = h2 as u32;
        let m3 = m3 as u32;

        let s = match cpu.borrow_wram().enemy_mon().status() {
            status if status.is_frozen() || status.is_sleeping() => 10,
            status if status.is_burned() || status.is_poisoned() || status.is_paralyzed() => 5,
            _ => 0,
        };

        // Gen 2 Catch Rate Formula
        catch_rate = u8::max((((m3 - h2) * (catch_rate as u32)) / m3) as u8, 1).saturating_add(s);

        cpu.b = cpu.borrow_wram().battle_mon().item().map_or(0, Into::into);
        macros::farcall::farcall(cpu, 0x0d, 0x7dd0); // GetItemHeldEffect

        let held_catch_chance = cpu.b == HELD_CATCH_CHANCE;
        let held_catch_chance_inc = cpu.c; // c is set seemingly randomly by GetItemHeldEffect

        if held_catch_chance {
            catch_rate = catch_rate.saturating_add(held_catch_chance_inc);
        }
    };

    cpu.borrow_wram_mut().set_final_catch_rate(catch_rate);
    log::info!("Your chance to catch the Pokémon is: ({catch_rate} + 1) / 256");

    cpu.call(0x2f8c); // Random
    let rng = cpu.a;

    cpu.b = catch_rate;
    cpu.d = catch_rate;

    if rng <= catch_rate {
        poke_ball_effect_catch(cpu, Some(cpu.borrow_wram().enemy_mon().species()));
    } else {
        poke_ball_effect_catch(cpu, None)
    }
}

fn poke_ball_effect_catch(cpu: &mut Cpu, species: Option<PokemonSpecies>) {
    cpu.borrow_wram_mut().set_wild_mon(species);

    cpu.c = 20;
    cpu.call(0x0468); // DelayFrames

    let battle_anim_param = match cpu.borrow_wram().cur_item() {
        Item::MasterBall => Item::MasterBall.into(),
        Item::UltraBall => Item::UltraBall.into(),
        Item::GreatBall => Item::GreatBall.into(),
        _ => Item::PokeBall.into(),
    };

    cpu.borrow_wram_mut()
        .set_battle_anim_param(battle_anim_param);

    cpu.borrow_wram_mut()
        .set_fx_anim_id(move_constants::ANIM_THROW_POKE_BALL);

    cpu.write_byte(hram::BATTLE_TURN, 0);
    cpu.borrow_wram_mut().set_thrown_ball_wobble_count(0);
    cpu.borrow_wram_mut().set_num_hits(0);
    macros::predef::predef_call!(cpu, PlayBattleAnim);

    cpu.a = cpu.borrow_wram().wild_mon().map_or(0, Into::into);

    if species.is_none() {
        match cpu.borrow_wram().thrown_ball_wobble_count() {
            1 => return poke_ball_effect_shake_and_break_free(cpu, 0x6db5), // BallBrokeFreeText
            2 => return poke_ball_effect_shake_and_break_free(cpu, 0x6dba), // BallAppearedCaughtText
            3 => return poke_ball_effect_shake_and_break_free(cpu, 0x6dbf), // BallAlmostHadItText
            4 => return poke_ball_effect_shake_and_break_free(cpu, 0x6dc4), // BallSoCloseText

            n => log::warn!("Unexpected wobble count: {n}"),
        }
    }

    let saved_status = cpu.borrow_wram().enemy_mon().status();
    let saved_hp = cpu.borrow_wram().enemy_mon().hp();
    let saved_item = cpu.borrow_wram().enemy_mon().item();

    let is_transformed = cpu.borrow_wram().enemy_sub_status_is_transformed();

    if !is_transformed {
        let dvs = cpu.borrow_wram().enemy_mon().dvs();
        cpu.borrow_wram_mut().set_enemy_backup_dvs(dvs);
    }

    let species = cpu.borrow_wram().temp_enemy_mon_species();
    cpu.borrow_wram_mut().set_cur_party_species(species);

    let cur_party_level = cpu.borrow_wram().enemy_mon().level();
    cpu.borrow_wram_mut().set_cur_party_level(cur_party_level);

    cpu.borrow_wram_mut()
        .set_enemy_sub_status_is_transformed(true);

    macros::farcall::farcall(cpu, 0x0f, 0x68eb); // LoadEnemyMon

    cpu.borrow_wram_mut()
        .set_enemy_sub_status_is_transformed(is_transformed);

    cpu.borrow_wram_mut()
        .enemy_mon_mut()
        .set_item(saved_item)
        .set_hp(saved_hp)
        .set_status(saved_status);

    if !is_transformed {
        cpu.set_hl(0xc735); // wWildMonMoves
        cpu.set_de(0xd208); // wEnemyMonMoves
        cpu.set_bc(battle_constants::NUM_MOVES as u16);
        cpu.call(0x3026); // CopyBytes

        cpu.set_hl(0xc739); // wWildMonPP
        cpu.set_de(0xd20e); // wEnemyMonPP
        cpu.set_bc(battle_constants::NUM_MOVES as u16);
        cpu.call(0x3026); // CopyBytes
    }

    let species = cpu.borrow_wram().enemy_mon().species();
    cpu.borrow_wram_mut().set_wild_mon(Some(species));
    cpu.borrow_wram_mut().set_cur_party_species(Some(species));
    cpu.borrow_wram_mut().set_temp_species(Some(species));

    if cpu.borrow_wram().battle_type() == BattleType::Tutorial {
        return poke_ball_effect_shake_and_break_free(cpu, 0x6dc9); // Text_GotchaMonWasCaught
    }

    macros::farcall::farcall(cpu, 0x41, 0x607f); // StubbedTrainerRankings_WildMonsCaught

    cpu.set_hl(0x6dc9); // Text_GotchaMonWasCaught
    cpu.call(0x1057); // PrintText

    cpu.call(0x300b); // ClearSprites

    let species = cpu.borrow_wram().temp_species();

    cpu.a = species.map_or(0, Into::into).wrapping_sub(1); // Index into wPokedexCaught
    cpu.call(0x3393); // CheckCaughtMon
    let saved_check_caught_mon = cpu.c;

    cpu.a = species.map_or(0, Into::into).wrapping_sub(1); // Index into wPokedexCaught
    cpu.call(0x3380); // SetSeenAndCaughtMon

    if saved_check_caught_mon == 0 {
        cpu.call(0x2ead); // CheckReceivedDex

        if !cpu.flag(CpuFlag::Z) {
            cpu.set_hl(0x6df0); // NewDexDataText
            cpu.call(0x1057); // PrintText
            cpu.call(0x300b); // ClearSprites

            let species = cpu.borrow_wram().enemy_mon().species();
            cpu.borrow_wram_mut().set_temp_species(Some(species));

            macros::predef::predef_call!(cpu, NewPokedexEntry);
        }
    }

    if cpu.borrow_wram().battle_type() == BattleType::Contest {
        macros::farcall::farcall(cpu, 0x03, 0x66ce); // BugContest_SetCaughtContestMon
        return poke_ball_effect_return_from_capture(cpu);
    }

    if cpu.borrow_wram().battle_type() == BattleType::Celebi {
        let mut value = cpu.borrow_wram().battle_result();
        value |= BattleResult::CAUGHT_CELEBI;
        cpu.borrow_wram_mut().set_battle_result(value);
    }

    if cpu.borrow_wram().party_count() == PARTY_LENGTH {
        poke_ball_effect_send_to_pc(cpu)
    } else {
        poke_ball_effect_add_to_party(cpu)
    }
}

fn poke_ball_effect_add_to_party(cpu: &mut Cpu) {
    cpu.borrow_wram_mut().set_mon_type(MonType::Party);

    cpu.call(0x300b); // ClearSprites
    macros::predef::predef_call!(cpu, TryAddMonToParty);
    macros::farcall::farcall(cpu, 0x13, 0x5b49); // SetCaughtData

    if cpu.borrow_wram().cur_item() == Item::FriendBall {
        let idx = cpu.borrow_wram().party_count() - 1;
        let base = 0xdcfa; // wPartyMon1Happiness
        let offset = base + (idx as u16 * PARTYMON_STRUCT_LENGTH as u16);

        cpu.write_byte(offset, FRIEND_BALL_HAPPINESS);
    }

    cpu.set_hl(0x6df5); // AskGiveNicknameText
    cpu.call(0x1057); // PrintText

    let named_object_index = cpu.borrow_wram().cur_party_species().map_or(0, Into::into);
    cpu.borrow_wram_mut()
        .set_named_object_index(named_object_index);

    cpu.call(0x343b); // GetPokemonName
    cpu.call(0x1dcf); // YesNoBox

    if !cpu.flag(CpuFlag::C) {
        let idx = cpu.borrow_wram().party_count() - 1;
        cpu.borrow_wram_mut().set_cur_party_mon(idx);

        let base = 0xde41; // wPartyMonNicknames
        let offset = base + (idx as u16 * text_constants::MON_NAME_LENGTH as u16);

        cpu.borrow_wram_mut().set_mon_type(MonType::Party);

        cpu.b = menu_constants::NAME_MON;
        cpu.set_de(offset);
        macros::farcall::farcall(cpu, 0x04, 0x56c1); // NamingScreen

        cpu.call(0x04b6); // RotateThreePalettesRight
        cpu.call(0x0e51); // LoadStandardFont

        cpu.set_hl(offset);
        cpu.set_de(0xd073); // wStringBuffer1
        cpu.call(0x2ef9); // InitName
    }

    poke_ball_effect_return_from_capture(cpu)
}

fn poke_ball_effect_send_to_pc(cpu: &mut Cpu) {
    cpu.call(0x300b); // ClearSprites

    macros::predef::predef_call!(cpu, SendMonIntoBox);
    macros::farcall::farcall(cpu, 0x13, 0x5b83); // SetBoxMonCaughtData

    cpu.a = 1; // BANK(sBoxCount)
    cpu.call(0x2fcb); // OpenSRAM

    cpu.a = cpu.read_byte(0xad10); // sBoxCount

    if cpu.a == MONS_PER_BOX {
        let mut value = cpu.borrow_wram().battle_result();
        value |= BattleResult::BOX_FULL;
        cpu.borrow_wram_mut().set_battle_result(value);
    }

    if cpu.borrow_wram().cur_item() == Item::FriendBall {
        // The captured mon is now first in the box
        cpu.write_byte(0xad41, FRIEND_BALL_HAPPINESS); // sBoxMon1Happiness
    }

    cpu.call(0x2fe1); // CloseSRAM

    cpu.set_hl(0x6df5); // AskGiveNicknameText
    cpu.call(0x1057); // PrintText

    let named_object_index = cpu.borrow_wram().cur_party_species().map_or(0, Into::into);
    cpu.borrow_wram_mut()
        .set_named_object_index(named_object_index);

    cpu.call(0x343b); // GetPokemonName
    cpu.call(0x1dcf); // YesNoBox

    if !cpu.flag(CpuFlag::C) {
        cpu.borrow_wram_mut().set_cur_party_mon(0);
        cpu.borrow_wram_mut().set_mon_type(MonType::Box);

        cpu.set_de(wram::MON_OR_ITEM_NAME_BUFFER);
        cpu.b = menu_constants::NAME_MON;
        macros::farcall::farcall(cpu, 0x04, 0x56c1); // NamingScreen

        cpu.a = sram::BOX_MON_NICKNAMES.0;
        cpu.call(0x2fcb); // OpenSRAM

        cpu.set_hl(wram::MON_OR_ITEM_NAME_BUFFER);
        cpu.set_de(sram::BOX_MON_NICKNAMES.1);
        cpu.set_bc(text_constants::MON_NAME_LENGTH as u16);
        cpu.call(0x3026); // CopyBytes

        cpu.set_hl(sram::BOX_MON_NICKNAMES.1);
        cpu.set_de(wram::STRING_BUFFER_1);
        cpu.call(0x2ef9); // InitName

        cpu.call(0x2fe1); // CloseSRAM
    }

    cpu.a = sram::BOX_MON_NICKNAMES.0;
    cpu.call(0x2fcb); // OpenSRAM

    cpu.set_hl(sram::BOX_MON_NICKNAMES.1);
    cpu.set_de(wram::MON_OR_ITEM_NAME_BUFFER);
    cpu.set_bc(text_constants::MON_NAME_LENGTH as u16);
    cpu.call(0x3026); // CopyBytes

    cpu.call(0x2fe1); // CloseSRAM

    cpu.set_hl(0x6deb); // BallSentToPCText
    cpu.call(0x1057); // PrintText

    cpu.call(0x04b6); // RotateThreePalettesRight
    cpu.call(0x0e51); // LoadStandardFont

    poke_ball_effect_return_from_capture(cpu)
}

fn poke_ball_effect_shake_and_break_free(cpu: &mut Cpu, text_ptr: u16) {
    cpu.set_hl(text_ptr);
    cpu.call(0x1057); // PrintText
    cpu.call(0x300b); // ClearSprites

    poke_ball_effect_return_from_capture(cpu);
}

fn poke_ball_effect_return_from_capture(cpu: &mut Cpu) {
    match cpu.borrow_wram().battle_type() {
        BattleType::Tutorial | BattleType::Debug => {
            // do nothing
        }

        BattleType::Contest => {
            let balls = cpu.borrow_wram().park_balls_remaining();
            let balls = balls.wrapping_sub(1);
            cpu.borrow_wram_mut().set_park_balls_remaining(balls);
        }

        _ => {
            if cpu.borrow_wram().wild_mon().is_some() {
                cpu.call(0x31f3); // ClearBGPalettes
                cpu.call(0x0fc8); // ClearTilemap
            }

            cpu.set_hl(wram::NUM_ITEMS);
            cpu.borrow_wram_mut().set_item_quantity_change(1);
            cpu.call(0x2f53); // TossItem
        }
    }

    cpu.pc = cpu.stack_pop(); // ret
}
