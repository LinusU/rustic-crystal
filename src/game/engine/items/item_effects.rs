use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::{self, BattleType, SUBSTATUS_TRANSFORMED},
            item_constants::Item,
            item_data_constants::HELD_CATCH_CHANCE,
            menu_constants, move_constants,
            pokemon_constants::PokemonSpecies,
            pokemon_data_constants::{
                FRIEND_BALL_HAPPINESS, MONS_PER_BOX, PARTYMON_STRUCT_LENGTH, PARTY_LENGTH,
            },
            ram_constants::{MonType, NO_TEXT_SCROLL},
            text_constants,
        },
        macros,
        ram::{hram, sram, wram},
    },
};

// BUG: The Dude's catching tutorial may crash if his Poké Ball can't be used
pub fn poke_ball_effect(cpu: &mut Cpu) {
    log::info!("poke_ball_effect()");

    cpu.pc = 0x68a2;

    // ld a, [wBattleMode]
    cpu.a = cpu.borrow_wram().battle_mode().map_or(0, Into::into);
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jp nz, UseBallInTrainerBattle
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        cpu.jump(0x77a0); // UseBallInTrainerBattle
        return;
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

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

    // jr nz, .room_in_party
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_room_in_party(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, BANK(sBoxCount)
    cpu.a = 0x01;
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

    // ld a, [sBoxCount]
    cpu.a = cpu.borrow_sram().box_count();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp MONS_PER_BOX
    cpu.set_flag(CpuFlag::Z, cpu.a == MONS_PER_BOX);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (MONS_PER_BOX & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < MONS_PER_BOX);
    cpu.pc += 2;
    cpu.cycle(8);

    // call CloseSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fe1); // CloseSRAM
        cpu.pc = pc;
    }

    // jp z, Ball_BoxIsFullMessage
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        cpu.jump(0x77dc); // Ball_BoxIsFullMessage
        return;
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    poke_ball_effect_room_in_party(cpu);
}

fn poke_ball_effect_room_in_party(cpu: &mut Cpu) {
    cpu.pc = 0x68c0;

    // BUG: Using a Park Ball in non-Contest battles has a corrupt animation (see docs/bugs_and_glitches.md)
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wWildMon], a
    cpu.borrow_wram_mut().set_wild_mon(0);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wCurItem]
    cpu.a = cpu.borrow_wram().cur_item();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp PARK_BALL
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(Item::ParkBall));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(Item::ParkBall) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(Item::ParkBall));
    cpu.pc += 2;
    cpu.cycle(8);

    // call nz, ReturnToBattle_UseBall
    if !cpu.flag(CpuFlag::Z) {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x6dfa); // ReturnToBattle_UseBall
        cpu.pc = pc;
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // ld hl, wOptions
    cpu.set_hl(wram::OPTIONS);
    cpu.pc += 3;
    cpu.cycle(12);

    // res NO_TEXT_SCROLL, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value & !(1 << NO_TEXT_SCROLL));
    }
    cpu.pc += 2;
    cpu.cycle(16);

    // ld hl, ItemUsedText
    cpu.set_hl(0x783d); // ItemUsedText
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

    // ld a, [wEnemyMonCatchRate]
    cpu.a = cpu.borrow_wram().enemy_mon_catch_rate();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [wBattleType]
    cpu.a = cpu.borrow_wram().battle_type();
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

    // jp z, .catch_without_fail
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return poke_ball_effect_catch_without_fail(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // ld a, [wCurItem]
    cpu.a = cpu.borrow_wram().cur_item();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp MASTER_BALL
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(Item::MasterBall));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(Item::MasterBall) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(Item::MasterBall));
    cpu.pc += 2;
    cpu.cycle(8);

    // jp z, .catch_without_fail
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return poke_ball_effect_catch_without_fail(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // ld a, [wCurItem]
    cpu.a = cpu.borrow_wram().cur_item();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld hl, BallMultiplierFunctionTable
    cpu.set_hl(0x6c0a); // BallMultiplierFunctionTable
    cpu.pc += 3;
    cpu.cycle(12);

    poke_ball_effect_get_multiplier_loop(cpu);
}

fn poke_ball_effect_get_multiplier_loop(cpu: &mut Cpu) {
    cpu.pc = 0x68f2;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // cp $ff
    cpu.set_flag(CpuFlag::Z, cpu.a == 0xff);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < 0x0f);
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0xff);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .skip_or_return_from_ball_fn
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_skip_or_return_from_ball_fn(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // cp c
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.c);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.c & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.c);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .call_ball_function
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_call_ball_function(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // jr .get_multiplier_loop
    cpu.cycle(12);
    poke_ball_effect_get_multiplier_loop(cpu)
}

fn poke_ball_effect_call_ball_function(cpu: &mut Cpu) {
    cpu.pc = 0x68fe;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld h, [hl]
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld l, a
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld de, .skip_or_return_from_ball_fn
    cpu.set_de(0x6906); // PokeBallEffect.skip_or_return_from_ball_fn
    cpu.pc += 3;
    cpu.cycle(12);

    // push de
    cpu.stack_push(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);

    // jp hl
    {
        let target = cpu.hl();
        cpu.cycle(4);
        cpu.jump(target);

        assert_eq!(cpu.pc, 0x6906); // PokeBallEffect.skip_or_return_from_ball_fn
        poke_ball_effect_skip_or_return_from_ball_fn(cpu)
    }
}

fn poke_ball_effect_skip_or_return_from_ball_fn(cpu: &mut Cpu) {
    cpu.pc = 0x6906;

    // ld a, [wCurItem]
    cpu.a = cpu.borrow_wram().cur_item();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp LEVEL_BALL
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(Item::LevelBall));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(Item::LevelBall) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(Item::LevelBall));
    cpu.pc += 2;
    cpu.cycle(8);

    // ld a, b
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);

    // jp z, .skip_hp_calc
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return poke_ball_effect_skip_hp_calc(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // ld a, b
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hMultiplicand + 2], a
    cpu.write_byte(hram::MULTIPLICAND + 2, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld hl, wEnemyMonHP
    cpu.set_hl(0xd216); // wEnemyMonHP
    cpu.pc += 3;
    cpu.cycle(12);

    // ld b, [hl]
    cpu.b = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld c, [hl]
    cpu.c = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld d, [hl]
    cpu.d = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld e, [hl]
    cpu.e = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // sla c
    cpu.set_flag(CpuFlag::C, (cpu.c & 0x80) != 0);
    cpu.c <<= 1;
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // rl b
    {
        let carry = if cpu.flag(CpuFlag::C) { 0x01 } else { 0 };
        cpu.set_flag(CpuFlag::C, (cpu.b & 0x80) != 0);
        cpu.b = (cpu.b << 1) | carry;
    }
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld h, d
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld l, e
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);

    // add hl, de
    {
        let hl = cpu.hl();
        let de = cpu.de();
        let result = hl.wrapping_add(de);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (de & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - de);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // add hl, de
    {
        let hl = cpu.hl();
        let de = cpu.de();
        let result = hl.wrapping_add(de);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (de & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - de);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld d, h
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld e, l
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, d
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .okay_1
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_okay_1(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // srl d
    cpu.set_flag(CpuFlag::C, cpu.d & 1 != 0);
    cpu.d >>= 1;
    cpu.set_flag(CpuFlag::Z, cpu.d == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // rr e
    {
        let carry = if cpu.flag(CpuFlag::C) { 0x80 } else { 0 };
        cpu.set_flag(CpuFlag::C, (cpu.e & 0x01) != 0);
        cpu.e = (cpu.e >> 1) | carry;
    }
    cpu.set_flag(CpuFlag::Z, cpu.e == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // srl d
    cpu.set_flag(CpuFlag::C, cpu.d & 1 != 0);
    cpu.d >>= 1;
    cpu.set_flag(CpuFlag::Z, cpu.d == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // rr e
    {
        let carry = if cpu.flag(CpuFlag::C) { 0x80 } else { 0 };
        cpu.set_flag(CpuFlag::C, (cpu.e & 0x01) != 0);
        cpu.e = (cpu.e >> 1) | carry;
    }
    cpu.set_flag(CpuFlag::Z, cpu.e == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // srl b
    cpu.set_flag(CpuFlag::C, cpu.b & 1 != 0);
    cpu.b >>= 1;
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // rr c
    {
        let carry = if cpu.flag(CpuFlag::C) { 0x80 } else { 0 };
        cpu.set_flag(CpuFlag::C, (cpu.c & 0x01) != 0);
        cpu.c = (cpu.c >> 1) | carry;
    }
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // srl b
    cpu.set_flag(CpuFlag::C, cpu.b & 1 != 0);
    cpu.b >>= 1;
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // rr c
    {
        let carry = if cpu.flag(CpuFlag::C) { 0x80 } else { 0 };
        cpu.set_flag(CpuFlag::C, (cpu.c & 0x01) != 0);
        cpu.c = (cpu.c >> 1) | carry;
    }
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld a, c
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .okay_1
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_okay_1(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld c, $1
    cpu.c = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);

    poke_ball_effect_okay_1(cpu);
}

fn poke_ball_effect_okay_1(cpu: &mut Cpu) {
    cpu.pc = 0x6940;

    // ld b, e
    cpu.b = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);

    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, b
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);

    // sub a, c
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.c & 0x0f));
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.c);
    cpu.a = cpu.a.wrapping_sub(cpu.c);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hMultiplier], a
    cpu.write_byte(hram::MULTIPLIER, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hDividend + 0], a
    cpu.write_byte(hram::DIVIDEND, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [hMultiplicand + 0], a
    cpu.write_byte(hram::MULTIPLICAND, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [hMultiplicand + 1], a
    cpu.write_byte(hram::MULTIPLICAND + 1, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // call Multiply
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3119); // Multiply
        cpu.pc = pc;
    }

    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld a, b
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hDivisor], a
    cpu.write_byte(hram::DIVISOR, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld b, 4
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);

    // call Divide
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3124); // Divide
        cpu.pc = pc;
    }

    // ldh a, [hQuotient + 3]
    cpu.a = cpu.read_byte(hram::QUOTIENT + 3);
    cpu.pc += 2;
    cpu.cycle(12);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .statuscheck
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_statuscheck(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, 1
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);

    poke_ball_effect_statuscheck(cpu);
}

fn poke_ball_effect_statuscheck(cpu: &mut Cpu) {
    cpu.pc = 0x6960;

    // BUG: BRN/PSN/PAR do not affect catch rate (see docs/bugs_and_glitches.md)
    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, [wEnemyMonStatus]
    cpu.a = cpu.borrow_wram().enemy_mon_status();
    cpu.pc += 3;
    cpu.cycle(16);

    // and 1 << FRZ | SLP_MASK
    cpu.a &= 0b100111;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld c, 10
    cpu.c = 10;
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .addstatus
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_addstatus(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld c, 5
    cpu.c = 5;
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .addstatus
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_addstatus(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld c, 0
    cpu.c = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    poke_ball_effect_addstatus(cpu);
}

fn poke_ball_effect_addstatus(cpu: &mut Cpu) {
    cpu.pc = 0x6971;

    // ld a, b
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);

    // add c
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.c & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.c as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.c);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nc, .max_1
    if !cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return poke_ball_effect_max_1(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, $ff
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);

    poke_ball_effect_max_1(cpu);
}

fn poke_ball_effect_max_1(cpu: &mut Cpu) {
    cpu.pc = 0x6977;

    // BUG: HELD_CATCH_CHANCE has no effect (see docs/bugs_and_glitches.md)
    // ld d, a
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // push de
    cpu.stack_push(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, [wBattleMonItem]
    cpu.a = cpu.borrow_wram().battle_mon_item();
    cpu.pc += 3;
    cpu.cycle(16);

    // farcall GetItemHeldEffect
    macros::farcall::farcall(cpu, 0x0d, 0x7dd0);

    // ld a, b
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);

    // cp HELD_CATCH_CHANCE
    cpu.set_flag(CpuFlag::Z, cpu.a == HELD_CATCH_CHANCE);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (HELD_CATCH_CHANCE & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < HELD_CATCH_CHANCE);
    cpu.pc += 2;
    cpu.cycle(8);

    // pop de
    {
        let de = cpu.stack_pop();
        cpu.set_de(de);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld a, d
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .max_2
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_max_2(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // add c
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) + (cpu.c & 0x0f) > 0x0f);
    cpu.set_flag(CpuFlag::C, (cpu.a as u16) + (cpu.c as u16) > 0xff);
    cpu.a = cpu.a.wrapping_add(cpu.c);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nc, .max_2
    if !cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return poke_ball_effect_max_2(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, $ff
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);

    poke_ball_effect_max_2(cpu);
}

fn poke_ball_effect_max_2(cpu: &mut Cpu) {
    cpu.pc = 0x698e;

    poke_ball_effect_skip_hp_calc(cpu);
}

fn poke_ball_effect_skip_hp_calc(cpu: &mut Cpu) {
    cpu.pc = 0x698e;

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wFinalCatchRate], a
    let final_catch_rate = cpu.a;
    cpu.borrow_wram_mut().set_final_catch_rate(final_catch_rate);
    cpu.pc += 3;
    cpu.cycle(16);

    // call Random
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2f8c); // Random
        cpu.pc = pc;
    }

    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, 0
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .catch_without_fail
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_catch_without_fail(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // jr nc, .fail_to_catch
    if !cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return poke_ball_effect_fail_to_catch(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    poke_ball_effect_catch_without_fail(cpu);
}

fn poke_ball_effect_catch_without_fail(cpu: &mut Cpu) {
    cpu.pc = 0x699c;

    // ld a, [wEnemyMonSpecies]
    cpu.a = cpu.borrow_wram().enemy_mon_species();
    cpu.pc += 3;
    cpu.cycle(16);

    poke_ball_effect_fail_to_catch(cpu);
}

fn poke_ball_effect_fail_to_catch(cpu: &mut Cpu) {
    cpu.pc = 0x699f;

    // ld [wWildMon], a
    let wild_mon = cpu.a;
    cpu.borrow_wram_mut().set_wild_mon(wild_mon);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld c, 20
    cpu.c = 20;
    cpu.pc += 2;
    cpu.cycle(8);

    // call DelayFrames
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0468); // DelayFrames
        cpu.pc = pc;
    }

    // ld a, [wCurItem]
    cpu.a = cpu.borrow_wram().cur_item();
    cpu.pc += 3;
    cpu.cycle(16);

    // Assumes Master/Ultra/Great come before
    // cp POKE_BALL + 1
    cpu.set_flag(CpuFlag::Z, cpu.a == (u8::from(Item::PokeBall) + 1));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < ((u8::from(Item::PokeBall) + 1) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < (u8::from(Item::PokeBall) + 1));
    cpu.pc += 2;
    cpu.cycle(8);

    // jr c, .not_kurt_ball
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return poke_ball_effect_not_kurt_ball(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, POKE_BALL
    cpu.a = Item::PokeBall.into();
    cpu.pc += 2;
    cpu.cycle(8);

    poke_ball_effect_not_kurt_ball(cpu);
}

fn poke_ball_effect_not_kurt_ball(cpu: &mut Cpu) {
    cpu.pc = 0x69b0;

    // ld [wBattleAnimParam], a
    let battle_anim_param = cpu.a;
    cpu.borrow_wram_mut()
        .set_battle_anim_param(battle_anim_param);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld de, ANIM_THROW_POKE_BALL
    cpu.set_de(move_constants::ANIM_THROW_POKE_BALL);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, e
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wFXAnimID], a
    cpu.write_byte(0xcfc2, cpu.a); // wFXAnimID + 0
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, d
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wFXAnimID + 1], a
    cpu.write_byte(0xcfc3, cpu.a); // wFXAnimID + 1
    cpu.pc += 3;
    cpu.cycle(16);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hBattleTurn], a
    cpu.write_byte(hram::BATTLE_TURN, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld [wThrownBallWobbleCount], a
    let thrown_ball_wobble_count = cpu.a;
    cpu.borrow_wram_mut()
        .set_thrown_ball_wobble_count(thrown_ball_wobble_count);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wNumHits], a
    let num_hits = cpu.a;
    cpu.borrow_wram_mut().set_num_hits(num_hits);
    cpu.pc += 3;
    cpu.cycle(16);

    // predef PlayBattleAnim
    macros::predef::predef_call!(cpu, PlayBattleAnim);

    // ld a, [wWildMon]
    cpu.a = cpu.borrow_wram().wild_mon();
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .caught
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_caught(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [wThrownBallWobbleCount]
    cpu.a = cpu.borrow_wram().thrown_ball_wobble_count();
    cpu.pc += 3;
    cpu.cycle(16);

    // cp 1
    cpu.set_flag(CpuFlag::Z, cpu.a == 1);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (1 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 1);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld hl, BallBrokeFreeText
    cpu.set_hl(0x6db5); // BallBrokeFreeText
    cpu.pc += 3;
    cpu.cycle(12);

    // jp z, .shake_and_break_free
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return poke_ball_effect_shake_and_break_free(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // cp 2
    cpu.set_flag(CpuFlag::Z, cpu.a == 2);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (2 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 2);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld hl, BallAppearedCaughtText
    cpu.set_hl(0x6dba); // BallAppearedCaughtText
    cpu.pc += 3;
    cpu.cycle(12);

    // jp z, .shake_and_break_free
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return poke_ball_effect_shake_and_break_free(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // cp 3
    cpu.set_flag(CpuFlag::Z, cpu.a == 3);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (3 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 3);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld hl, BallAlmostHadItText
    cpu.set_hl(0x6dbf); // BallAlmostHadItText
    cpu.pc += 3;
    cpu.cycle(12);

    // jp z, .shake_and_break_free
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return poke_ball_effect_shake_and_break_free(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // cp 4
    cpu.set_flag(CpuFlag::Z, cpu.a == 4);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (4 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 4);
    cpu.pc += 2;
    cpu.cycle(8);

    // ld hl, BallSoCloseText
    cpu.set_hl(0x6dc4); // BallSoCloseText
    cpu.pc += 3;
    cpu.cycle(12);

    // jp z, .shake_and_break_free
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return poke_ball_effect_shake_and_break_free(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    poke_ball_effect_caught(cpu);
}

fn poke_ball_effect_caught(cpu: &mut Cpu) {
    cpu.pc = 0x69f5;

    // ld hl, wEnemyMonStatus
    cpu.set_hl(0xd214); // wEnemyMonStatus
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld hl, wEnemyMonItem
    cpu.set_hl(0xd207); // wEnemyMonItem
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld hl, wEnemySubStatus5
    cpu.set_hl(0xc671); // wEnemySubStatus5
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // set SUBSTATUS_TRANSFORMED, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value | (1 << SUBSTATUS_TRANSFORMED));
    }
    cpu.pc += 2;
    cpu.cycle(16);

    // BUG: Catching a Transformed Pokémon always catches a Ditto (see docs/bugs_and_glitches.md)
    // bit SUBSTATUS_TRANSFORMED, a
    cpu.set_flag(CpuFlag::Z, (cpu.a & (1 << SUBSTATUS_TRANSFORMED)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .ditto
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_ditto(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // jr .not_ditto
    cpu.cycle(12);
    poke_ball_effect_not_ditto(cpu)
}

fn poke_ball_effect_ditto(cpu: &mut Cpu) {
    cpu.pc = 0x6a13;

    // ld a, DITTO
    cpu.a = PokemonSpecies::Ditto.into();
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wTempEnemyMonSpecies], a
    let temp_enemy_mon_species = cpu.a;
    cpu.borrow_wram_mut()
        .set_temp_enemy_mon_species(temp_enemy_mon_species);
    cpu.pc += 3;
    cpu.cycle(16);

    // jr .load_data
    cpu.cycle(12);
    poke_ball_effect_load_data(cpu)
}

fn poke_ball_effect_not_ditto(cpu: &mut Cpu) {
    cpu.pc = 0x6a1a;

    // set SUBSTATUS_TRANSFORMED, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.write_byte(cpu.hl(), value | (1 << SUBSTATUS_TRANSFORMED));
    }
    cpu.pc += 2;
    cpu.cycle(16);

    // ld hl, wEnemyBackupDVs
    cpu.set_hl(0xc6f2); // wEnemyBackupDVs
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [wEnemyMonDVs]
    cpu.a = cpu.read_byte(0xd20c); // wEnemyMonDVs + 0
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [hli], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wEnemyMonDVs + 1]
    cpu.a = cpu.read_byte(0xd20d); // wEnemyMonDVs + 1
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    poke_ball_effect_load_data(cpu);
}

fn poke_ball_effect_load_data(cpu: &mut Cpu) {
    cpu.pc = 0x6a27;

    // ld a, [wTempEnemyMonSpecies]
    cpu.a = cpu.borrow_wram().temp_enemy_mon_species();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wCurPartySpecies], a
    let cur_party_species = cpu.a;
    cpu.borrow_wram_mut()
        .set_cur_party_species(cur_party_species);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wEnemyMonLevel]
    cpu.a = cpu.borrow_wram().enemy_mon_level();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wCurPartyLevel], a
    let cur_party_level = cpu.a;
    cpu.borrow_wram_mut().set_cur_party_level(cur_party_level);
    cpu.pc += 3;
    cpu.cycle(16);

    // farcall LoadEnemyMon
    macros::farcall::farcall(cpu, 0x0f, 0x68eb);

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [wEnemySubStatus5], a
    cpu.write_byte(0xc671, cpu.a); // wEnemySubStatus5
    cpu.pc += 3;
    cpu.cycle(16);

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // pop hl
    {
        let hl = cpu.stack_pop();
        cpu.set_hl(hl);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [hld], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [hld], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl() - 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // dec hl
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld [hl], a
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld hl, wEnemySubStatus5
    cpu.set_hl(0xc671); // wEnemySubStatus5
    cpu.pc += 3;
    cpu.cycle(12);

    // bit SUBSTATUS_TRANSFORMED, [hl]
    let value = cpu.read_byte(cpu.hl());
    cpu.set_flag(CpuFlag::Z, (value & (1 << SUBSTATUS_TRANSFORMED)) == 0);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 2;
    cpu.cycle(16);

    // jr nz, .Transformed
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_transformed(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld hl, wWildMonMoves
    cpu.set_hl(0xc735); // wWildMonMoves
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wEnemyMonMoves
    cpu.set_de(0xd208); // wEnemyMonMoves
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, NUM_MOVES
    cpu.set_bc(battle_constants::NUM_MOVES as u16);
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

    // ld hl, wWildMonPP
    cpu.set_hl(0xc739); // wWildMonPP
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wEnemyMonPP
    cpu.set_de(0xd20e); // wEnemyMonPP
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, NUM_MOVES
    cpu.set_bc(battle_constants::NUM_MOVES as u16);
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

    poke_ball_effect_transformed(cpu);
}

fn poke_ball_effect_transformed(cpu: &mut Cpu) {
    cpu.pc = 0x6a67;

    // ld a, [wEnemyMonSpecies]
    cpu.a = cpu.borrow_wram().enemy_mon_species();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wWildMon], a
    let wild_mon = cpu.a;
    cpu.borrow_wram_mut().set_wild_mon(wild_mon);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wCurPartySpecies], a
    let cur_party_species = cpu.a;
    cpu.borrow_wram_mut()
        .set_cur_party_species(cur_party_species);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wTempSpecies], a
    let temp_species = cpu.a;
    cpu.borrow_wram_mut().set_temp_species(temp_species);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, [wBattleType]
    cpu.a = cpu.borrow_wram().battle_type();
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

    // jp z, .FinishTutorial
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(16);
        return poke_ball_effect_finish_tutorial(cpu);
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // farcall StubbedTrainerRankings_WildMonsCaught
    macros::farcall::farcall(cpu, 0x41, 0x607f);

    // ld hl, Text_GotchaMonWasCaught
    cpu.set_hl(0x6dc9); // Text_GotchaMonWasCaught
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

    // call ClearSprites
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x300b); // ClearSprites
        cpu.pc = pc;
    }

    // ld a, [wTempSpecies]
    cpu.a = cpu.borrow_wram().temp_species();
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // call CheckCaughtMon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3393); // CheckCaughtMon
        cpu.pc = pc;
    }

    // ld a, c
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, [wTempSpecies]
    cpu.a = cpu.borrow_wram().temp_species();
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // call SetSeenAndCaughtMon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3380); // SetSeenAndCaughtMon
        cpu.pc = pc;
    }

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .skip_pokedex
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_skip_pokedex(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call CheckReceivedDex
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2ead); // CheckReceivedDex
        cpu.pc = pc;
    }

    // jr z, .skip_pokedex
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return poke_ball_effect_skip_pokedex(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld hl, NewDexDataText
    cpu.set_hl(0x6df0); // NewDexDataText
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

    // call ClearSprites
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x300b); // ClearSprites
        cpu.pc = pc;
    }

    // ld a, [wEnemyMonSpecies]
    cpu.a = cpu.borrow_wram().enemy_mon_species();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wTempSpecies], a
    let temp_species = cpu.a;
    cpu.borrow_wram_mut().set_temp_species(temp_species);
    cpu.pc += 3;
    cpu.cycle(16);

    // predef NewPokedexEntry
    macros::predef::predef_call!(cpu, NewPokedexEntry);

    poke_ball_effect_skip_pokedex(cpu);
}

fn poke_ball_effect_skip_pokedex(cpu: &mut Cpu) {
    cpu.pc = 0x6ab7;

    // ld a, [wBattleType]
    cpu.a = cpu.borrow_wram().battle_type();
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
    cpu.a = cpu.borrow_wram().cur_item();
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
    cpu.a = cpu.borrow_wram().cur_item();
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

fn poke_ball_effect_finish_tutorial(cpu: &mut Cpu) {
    cpu.pc = 0x6bd9;

    // ld hl, Text_GotchaMonWasCaught
    cpu.set_hl(0x6dc9); // Text_GotchaMonWasCaught
    cpu.pc += 3;
    cpu.cycle(12);

    poke_ball_effect_shake_and_break_free(cpu);
}

fn poke_ball_effect_shake_and_break_free(cpu: &mut Cpu) {
    cpu.pc = 0x6bdc;

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
    cpu.a = cpu.borrow_wram().battle_type();
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
    cpu.a = cpu.borrow_wram().wild_mon();
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
