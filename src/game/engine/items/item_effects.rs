use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::{self, BattleType},
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

    // BUG: Using a Park Ball in non-Contest battles has a corrupt animation (see docs/bugs_and_glitches.md)
    if cpu.borrow_wram().cur_item() != Item::ParkBall {
        cpu.call(0x6dfa); // ReturnToBattle_UseBall
    }

    cpu.borrow_wram_mut().set_no_text_scroll(false);

    cpu.set_hl(0x783d); // ItemUsedText
    cpu.call(0x1057); // PrintText

    if cpu.borrow_wram().battle_type() == BattleType::Tutorial {
        return poke_ball_effect_catch_without_fail(cpu);
    }

    cpu.b = cpu.borrow_wram().enemy_mon_catch_rate();

    log::trace!("base catch rate: {}", cpu.b);
    log::trace!("ball: {:?}", cpu.borrow_wram().cur_item());

    match cpu.borrow_wram().cur_item() {
        Item::MasterBall => {
            return poke_ball_effect_catch_without_fail(cpu);
        }

        Item::UltraBall => cpu.call(0x6c29), // UltraBallMultiplier
        Item::GreatBall => cpu.call(0x6c2f), // GreatBallMultiplier
        SAFARI_BALL => cpu.call(0x6c2f), // SafariBallMultiplier ; Safari Ball, leftover from RBY
        Item::HeavyBall => cpu.call(0x6c50), // HeavyBallMultiplier
        Item::LevelBall => cpu.call(0x6d8c), // LevelBallMultiplier
        Item::LureBall => cpu.call(0x6ccc), // LureBallMultiplier
        Item::FastBall => cpu.call(0x6d68), // FastBallMultiplier
        Item::MoonBall => cpu.call(0x6cdd), // MoonBallMultiplier
        Item::LoveBall => cpu.call(0x6d12), // LoveBallMultiplier
        Item::ParkBall => cpu.call(0x6c2f), // ParkBallMultiplier

        Item::PokeBall => {} // no special multiplier

        n => log::warn!("Unknown item used as ball: {n:?}"),
    }

    let catch_rate = cpu.b;
    log::trace!("catch rate after ball effect: {catch_rate}");

    if cpu.borrow_wram().cur_item() == Item::LevelBall {
        return poke_ball_effect_skip_hp_calc(cpu, catch_rate);
    }

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

        // BUG: BRN/PSN/PAR do not affect catch rate (see docs/bugs_and_glitches.md)
        status if status.is_burned() || status.is_poisoned() || status.is_paralyzed() => 0,

        _ => 0,
    };

    // Gen 2 Catch Rate Formula
    let x = u8::max((((m3 - h2) * (catch_rate as u32)) / m3) as u8, 1).saturating_add(s);

    log::info!("Your chance to catch the Pokémon is: ({x} + 1) / 256");

    // BUG: farcall overwrites a, and GetItemHeldEffect takes b anyway.
    // This is probably the reason the HELD_CATCH_CHANCE effect is never used.
    cpu.a = cpu.borrow_wram().battle_mon_item();
    cpu.b = x;
    macros::farcall::farcall(cpu, 0x0d, 0x7dd0); // GetItemHeldEffect

    let held_catch_chance = cpu.b == HELD_CATCH_CHANCE;
    let held_catch_chance_inc = cpu.c; // c is set seemingly randomly by GetItemHeldEffect

    if held_catch_chance {
        poke_ball_effect_skip_hp_calc(cpu, x.saturating_add(held_catch_chance_inc))
    } else {
        poke_ball_effect_skip_hp_calc(cpu, x)
    }
}

fn poke_ball_effect_skip_hp_calc(cpu: &mut Cpu, final_catch_rate: u8) {
    cpu.borrow_wram_mut().set_final_catch_rate(final_catch_rate);

    cpu.call(0x2f8c); // Random
    let rng = cpu.a;

    cpu.b = final_catch_rate;
    cpu.d = final_catch_rate;

    if rng <= final_catch_rate {
        poke_ball_effect_catch_without_fail(cpu)
    } else {
        poke_ball_effect_fail_to_catch(cpu, None)
    }
}

fn poke_ball_effect_catch_without_fail(cpu: &mut Cpu) {
    poke_ball_effect_fail_to_catch(cpu, Some(cpu.borrow_wram().enemy_mon().species()));
}

fn poke_ball_effect_fail_to_catch(cpu: &mut Cpu, species: Option<PokemonSpecies>) {
    cpu.pc = 0x699f;

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

    // BUG: Catching a Transformed Pokémon always catches a Ditto (see docs/bugs_and_glitches.md)
    if is_transformed {
        cpu.borrow_wram_mut()
            .set_temp_enemy_mon_species(Some(PokemonSpecies::Ditto));
    } else {
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

    // .transformed
    cpu.pc = 0x6a67;

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

    if saved_check_caught_mon != 0 {
        return poke_ball_effect_skip_pokedex(cpu);
    }

    cpu.call(0x2ead); // CheckReceivedDex

    if cpu.flag(CpuFlag::Z) {
        return poke_ball_effect_skip_pokedex(cpu);
    }

    cpu.set_hl(0x6df0); // NewDexDataText
    cpu.call(0x1057); // PrintText
    cpu.call(0x300b); // ClearSprites

    let species = cpu.borrow_wram().enemy_mon().species();
    cpu.borrow_wram_mut().set_temp_species(Some(species));

    macros::predef::predef_call!(cpu, NewPokedexEntry);

    poke_ball_effect_skip_pokedex(cpu);
}

fn poke_ball_effect_skip_pokedex(cpu: &mut Cpu) {
    cpu.pc = 0x6ab7;

    // ld a, [wBattleType]
    cpu.a = cpu.borrow_wram().battle_type().into();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp BATTLETYPE_CONTEST
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(BattleType::Contest));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(BattleType::Contest) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(BattleType::Contest));
    cpu.pc += 2;
    cpu.cycle(8);

    // jp z, .catch_bug_contest_mon
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return poke_ball_effect_catch_bug_contest_mon(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // cp BATTLETYPE_CELEBI
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(BattleType::Celebi));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(BattleType::Celebi) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(BattleType::Celebi));
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .not_celebi
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_not_celebi(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld hl, wBattleResult
    cpu.set_hl(0xd0ee); // wBattleResult
    cpu.pc += 3;
    cpu.cycle(12);

    // set BATTLERESULT_CAUGHT_CELEBI, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value | (1 << 6));
    }
    cpu.pc += 2;
    cpu.cycle(16);

    poke_ball_effect_not_celebi(cpu);
}

fn poke_ball_effect_not_celebi(cpu: &mut Cpu) {
    cpu.pc = 0x6ac8;

    // ld a, [wPartyCount]
    cpu.a = cpu.borrow_wram().party_count();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp PARTY_LENGTH
    cpu.set_flag(CpuFlag::Z, cpu.a == PARTY_LENGTH);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (PARTY_LENGTH & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < PARTY_LENGTH);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .SendToPC
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_send_to_pc(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // PARTYMON
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wMonType], a
    let mon_type = cpu.a;
    cpu.borrow_wram_mut().set_mon_type(mon_type);
    cpu.pc += 3;
    cpu.cycle(16);

    // call ClearSprites
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x300b); // ClearSprites
        cpu.pc = pc;
    }

    // predef TryAddMonToParty
    macros::predef::predef_call!(cpu, TryAddMonToParty);

    // farcall SetCaughtData
    macros::farcall::farcall(cpu, 0x13, 0x5b49);

    // ld a, [wCurItem]
    cpu.a = cpu.borrow_wram().cur_item().into();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp FRIEND_BALL
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(Item::FriendBall));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(Item::FriendBall) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(Item::FriendBall));
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .SkipPartyMonFriendBall
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_skip_party_mon_friend_ball(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [wPartyCount]
    cpu.a = cpu.borrow_wram().party_count();
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld hl, wPartyMon1Happiness
    cpu.set_hl(0xdcfa); // wPartyMon1Happiness
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, PARTYMON_STRUCT_LENGTH
    cpu.set_bc(PARTYMON_STRUCT_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);

    // call AddNTimes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x30fe); // AddNTimes
        cpu.pc = pc;
    }

    // ld a, FRIEND_BALL_HAPPINESS
    cpu.a = FRIEND_BALL_HAPPINESS;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    poke_ball_effect_skip_party_mon_friend_ball(cpu);
}

fn poke_ball_effect_skip_party_mon_friend_ball(cpu: &mut Cpu) {
    cpu.pc = 0x6af8;

    // ld hl, AskGiveNicknameText
    cpu.set_hl(0x6df5); // AskGiveNicknameText
    cpu.pc += 3;
    cpu.cycle(12);

    // call PrintText
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1057); // PrintText
        cpu.pc = pc;
    }

    // ld a, [wCurPartySpecies]
    cpu.a = cpu.borrow_wram().cur_party_species();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wNamedObjectIndex], a
    let named_object_index = cpu.a;
    cpu.borrow_wram_mut()
        .set_named_object_index(named_object_index);
    cpu.pc += 3;
    cpu.cycle(16);

    // call GetPokemonName
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x343b); // GetPokemonName
        cpu.pc = pc;
    }

    // call YesNoBox
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1dcf); // YesNoBox
        cpu.pc = pc;
    }

    // jp c, .return_from_capture
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(16);
        return poke_ball_effect_return_from_capture(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // ld a, [wPartyCount]
    cpu.a = cpu.borrow_wram().party_count();
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wCurPartyMon], a
    let cur_party_mon = cpu.a;
    cpu.borrow_wram_mut().set_cur_party_mon(cur_party_mon);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld hl, wPartyMonNicknames
    cpu.set_hl(0xde41); // wPartyMonNicknames
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, MON_NAME_LENGTH
    cpu.set_bc(text_constants::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);

    // call AddNTimes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x30fe); // AddNTimes
        cpu.pc = pc;
    }

    // ld d, h
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld e, l
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);

    // push de
    cpu.stack_push(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);

    // PARTYMON
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wMonType], a
    let mon_type = cpu.a;
    cpu.borrow_wram_mut().set_mon_type(mon_type);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld b, NAME_MON
    cpu.b = menu_constants::NAME_MON;
    cpu.pc += 2;
    cpu.cycle(8);

    // farcall NamingScreen
    macros::farcall::farcall(cpu, 0x04, 0x56c1);

    // call RotateThreePalettesRight
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x04b6); // RotateThreePalettesRight
        cpu.pc = pc;
    }

    // call LoadStandardFont
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0e51); // LoadStandardFont
        cpu.pc = pc;
    }

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld de, wStringBuffer1
    cpu.set_de(0xd073); // wStringBuffer1
    cpu.pc += 3;
    cpu.cycle(12);

    // call InitName
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2ef9); // InitName
        cpu.pc = pc;
    }

    // jp .return_from_capture
    cpu.cycle(16);
    poke_ball_effect_return_from_capture(cpu)
}

fn poke_ball_effect_send_to_pc(cpu: &mut Cpu) {
    cpu.pc = 0x6b3c;

    // call ClearSprites
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x300b); // ClearSprites
        cpu.pc = pc;
    }

    // predef SendMonIntoBox
    macros::predef::predef_call!(cpu, SendMonIntoBox);

    // farcall SetBoxMonCaughtData
    macros::farcall::farcall(cpu, 0x13, 0x5b83);

    // ld a, BANK(sBoxCount)
    cpu.a = cpu.read_byte(0x01); // BANK(sBoxCount)
    cpu.pc += 3;
    cpu.cycle(16);

    // call OpenSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fcb); // OpenSRAM
        cpu.pc = pc;
    }

    // ld a, [sBoxCount]
    cpu.a = cpu.read_byte(0xad10); // sBoxCount
    cpu.pc += 3;
    cpu.cycle(16);

    // cp MONS_PER_BOX
    cpu.set_flag(CpuFlag::Z, cpu.a == MONS_PER_BOX);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (MONS_PER_BOX & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < MONS_PER_BOX);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .BoxNotFullYet
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_box_not_full_yet(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld hl, wBattleResult
    cpu.set_hl(0xd0ee); // wBattleResult
    cpu.pc += 3;
    cpu.cycle(12);

    // set BATTLERESULT_BOX_FULL, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value | (1 << 7)); // BATTLERESULT_BOX_FULL
    }
    cpu.pc += 2;
    cpu.cycle(16);

    poke_ball_effect_box_not_full_yet(cpu);
}

fn poke_ball_effect_box_not_full_yet(cpu: &mut Cpu) {
    cpu.pc = 0x6b5b;

    // ld a, [wCurItem]
    cpu.a = cpu.borrow_wram().cur_item().into();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp FRIEND_BALL
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(Item::FriendBall));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(Item::FriendBall) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(Item::FriendBall));
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .SkipBoxMonFriendBall
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_skip_box_mon_friend_ball(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // The captured mon is now first in the box
    // ld a, FRIEND_BALL_HAPPINESS
    cpu.a = FRIEND_BALL_HAPPINESS;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [sBoxMon1Happiness], a
    cpu.write_byte(0xad41, cpu.a); // sBoxMon1Happiness
    cpu.pc += 3;
    cpu.cycle(16);

    poke_ball_effect_skip_box_mon_friend_ball(cpu);
}

fn poke_ball_effect_skip_box_mon_friend_ball(cpu: &mut Cpu) {
    cpu.pc = 0x6b67;

    // call CloseSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fe1); // CloseSRAM
        cpu.pc = pc;
    }

    // ld hl, AskGiveNicknameText
    cpu.set_hl(0x6df5); // AskGiveNicknameText
    cpu.pc += 3;
    cpu.cycle(12);

    // call PrintText
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1057); // PrintText
        cpu.pc = pc;
    }

    // ld a, [wCurPartySpecies]
    cpu.a = cpu.borrow_wram().cur_party_species();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wNamedObjectIndex], a
    let named_object_index = cpu.a;
    cpu.borrow_wram_mut()
        .set_named_object_index(named_object_index);
    cpu.pc += 3;
    cpu.cycle(16);

    // call GetPokemonName
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x343b); // GetPokemonName
        cpu.pc = pc;
    }

    // call YesNoBox
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1dcf); // YesNoBox
        cpu.pc = pc;
    }

    // jr c, .SkipBoxMonNickname
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return poke_ball_effect_skip_box_mon_nickname(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wCurPartyMon], a
    let cur_party_mon = cpu.a;
    cpu.borrow_wram_mut().set_cur_party_mon(cur_party_mon);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, BOXMON
    cpu.a = MonType::Box.into();
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wMonType], a
    let mon_type = cpu.a;
    cpu.borrow_wram_mut().set_mon_type(mon_type);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld de, wMonOrItemNameBuffer
    cpu.set_de(wram::MON_OR_ITEM_NAME_BUFFER);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld b, NAME_MON
    cpu.b = menu_constants::NAME_MON;
    cpu.pc += 2;
    cpu.cycle(8);

    // farcall NamingScreen
    macros::farcall::farcall(cpu, 0x04, 0x56c1);

    // ld a, BANK(sBoxMonNicknames)
    cpu.a = sram::BOX_MON_NICKNAMES.0;
    cpu.pc += 2;
    cpu.cycle(8);

    // call OpenSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fcb); // OpenSRAM
        cpu.pc = pc;
    }

    // ld hl, wMonOrItemNameBuffer
    cpu.set_hl(wram::MON_OR_ITEM_NAME_BUFFER);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, sBoxMonNicknames
    cpu.set_de(sram::BOX_MON_NICKNAMES.1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, MON_NAME_LENGTH
    cpu.set_bc(text_constants::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyBytes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3026); // CopyBytes
        cpu.pc = pc;
    }

    // ld hl, sBoxMonNicknames
    cpu.set_hl(sram::BOX_MON_NICKNAMES.1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wStringBuffer1
    cpu.set_de(wram::STRING_BUFFER_1);
    cpu.pc += 3;
    cpu.cycle(12);

    // call InitName
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2ef9); // InitName
        cpu.pc = pc;
    }

    // call CloseSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fe1); // CloseSRAM
        cpu.pc = pc;
    }

    poke_ball_effect_skip_box_mon_nickname(cpu);
}

fn poke_ball_effect_skip_box_mon_nickname(cpu: &mut Cpu) {
    cpu.pc = 0x6baf;

    // ld a, BANK(sBoxMonNicknames)
    cpu.a = sram::BOX_MON_NICKNAMES.0;
    cpu.pc += 2;
    cpu.cycle(8);

    // call OpenSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fcb); // OpenSRAM
        cpu.pc = pc;
    }

    // ld hl, sBoxMonNicknames
    cpu.set_hl(sram::BOX_MON_NICKNAMES.1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wMonOrItemNameBuffer
    cpu.set_de(wram::MON_OR_ITEM_NAME_BUFFER);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, MON_NAME_LENGTH
    cpu.set_bc(text_constants::MON_NAME_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyBytes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3026); // CopyBytes
        cpu.pc = pc;
    }

    // call CloseSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fe1); // CloseSRAM
        cpu.pc = pc;
    }

    // ld hl, BallSentToPCText
    cpu.set_hl(0x6deb); // BallSentToPCText
    cpu.pc += 3;
    cpu.cycle(12);

    // call PrintText
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1057); // PrintText
        cpu.pc = pc;
    }

    // call RotateThreePalettesRight
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x04b6); // RotateThreePalettesRight
        cpu.pc = pc;
    }

    // call LoadStandardFont
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0e51); // LoadStandardFont
        cpu.pc = pc;
    }

    // jr .return_from_capture
    cpu.cycle(12);
    poke_ball_effect_return_from_capture(cpu)
}

fn poke_ball_effect_catch_bug_contest_mon(cpu: &mut Cpu) {
    cpu.pc = 0x6bd1;

    // farcall BugContest_SetCaughtContestMon
    macros::farcall::farcall(cpu, 0x03, 0x66ce);

    // jr .return_from_capture
    cpu.cycle(12);
    poke_ball_effect_return_from_capture(cpu)
}

fn poke_ball_effect_shake_and_break_free(cpu: &mut Cpu, text_ptr: u16) {
    cpu.pc = 0x6bdc;

    cpu.set_hl(text_ptr);

    // call PrintText
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x1057); // PrintText
        cpu.pc = pc;
    }

    // call ClearSprites
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x300b); // ClearSprites
        cpu.pc = pc;
    }

    poke_ball_effect_return_from_capture(cpu);
}

fn poke_ball_effect_return_from_capture(cpu: &mut Cpu) {
    cpu.pc = 0x6be2;

    // ld a, [wBattleType]
    cpu.a = cpu.borrow_wram().battle_type().into();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp BATTLETYPE_TUTORIAL
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(BattleType::Tutorial));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(BattleType::Tutorial) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(BattleType::Tutorial));
    cpu.pc += 2;
    cpu.cycle(8);

    // ret z
    if cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // cp BATTLETYPE_DEBUG
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(BattleType::Debug));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(BattleType::Debug) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(BattleType::Debug));
    cpu.pc += 2;
    cpu.cycle(8);

    // ret z
    if cpu.flag(CpuFlag::Z) {
        cpu.pc = cpu.stack_pop();
        cpu.cycle(20);
        return;
    } else {
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // cp BATTLETYPE_CONTEST
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(BattleType::Contest));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(BattleType::Contest) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(BattleType::Contest));
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .used_park_ball
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_used_park_ball(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [wWildMon]
    cpu.a = cpu.borrow_wram().wild_mon().map_or(0, Into::into);
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .toss
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_toss(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call ClearBGPalettes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x31f3); // ClearBGPalettes
        cpu.pc = pc;
    }

    // call ClearTilemap
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0fc8); // ClearTilemap
        cpu.pc = pc;
    }

    poke_ball_effect_toss(cpu);
}

fn poke_ball_effect_toss(cpu: &mut Cpu) {
    cpu.pc = 0x6bfb;

    // ld hl, wNumItems
    cpu.set_hl(wram::NUM_ITEMS);
    cpu.pc += 3;
    cpu.cycle(12);

    // inc a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x0f);
    cpu.a = cpu.a.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wItemQuantityChange], a
    let item_quantity_change = cpu.a;
    cpu.borrow_wram_mut()
        .set_item_quantity_change(item_quantity_change);
    cpu.pc += 3;
    cpu.cycle(16);

    // jp TossItem
    cpu.cycle(16);
    cpu.jump(0x2f53); // TossItem
}

fn poke_ball_effect_used_park_ball(cpu: &mut Cpu) {
    cpu.pc = 0x6c05;

    // ld hl, wParkBallsRemaining
    cpu.set_hl(0xdc79); // wParkBallsRemaining
    cpu.pc += 3;
    cpu.cycle(12);

    // dec [hl]
    {
        let addr = cpu.hl();
        let value = cpu.read_byte(addr);
        cpu.write_byte(addr, value.wrapping_sub(1));
        cpu.set_flag(CpuFlag::Z, value == 1);
        cpu.set_flag(CpuFlag::H, (value & 0x0f) == 0);
        cpu.set_flag(CpuFlag::N, true);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
