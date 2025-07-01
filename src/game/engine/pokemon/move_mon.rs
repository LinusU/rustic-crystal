use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::{NUM_EXP_STATS, NUM_MOVES},
            pokemon_constants::PokemonSpecies,
            pokemon_data_constants::{BASE_HAPPINESS, MONS_PER_BOX},
            text_constants::{MON_NAME_LENGTH, NAME_LENGTH},
        },
        macros,
        ram::{hram, sram, wram},
    },
};

/// Sends the mon into one of Bills Boxes \
/// the data comes mainly from 'wEnemyMon:'
pub fn send_mon_into_box(cpu: &mut Cpu) {
    log::info!("send_mon_into_box()");

    cpu.pc = 0x5e6e;

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

    // ld de, sBoxCount
    cpu.set_de(0xad10); // sBoxCount
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [de]
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);

    // cp MONS_PER_BOX
    cpu.set_flag(CpuFlag::Z, cpu.a == MONS_PER_BOX);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (MONS_PER_BOX & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < MONS_PER_BOX);
    cpu.pc += 2;
    cpu.cycle(8);

    // jp nc, .full
    if !cpu.flag(CpuFlag::C) {
        cpu.cycle(16);
        send_mon_into_box_full(cpu);
        return;
    } else {
        cpu.pc += 3;
        cpu.cycle(12);
    }

    // inc a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x0f);
    cpu.a = cpu.a.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wCurPartySpecies]
    cpu.a = cpu.borrow_wram().cur_party_species().map_or(0, Into::into);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wCurSpecies], a
    let cur_species = match cpu.a {
        0 => None,
        n => Some(n.into()),
    };
    cpu.borrow_wram_mut().set_cur_species(cur_species);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    send_mon_into_box_loop(cpu);
}

fn send_mon_into_box_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5e85;

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [de]
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld b, a
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, c
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld c, b
    cpu.c = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x0f);
    cpu.a = cpu.a.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .loop
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return send_mon_into_box_loop(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call GetBaseData
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3856); // GetBaseData
        cpu.pc = pc;
    }

    // call ShiftBoxMon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x5f47); // ShiftBoxMon
        cpu.pc = pc;
    }

    // ld hl, wPlayerName
    cpu.set_hl(0xd47d); // wPlayerName
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, sBoxMonOTs
    cpu.set_de(0xafa6); // sBoxMonOTs
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, NAME_LENGTH
    cpu.set_bc(NAME_LENGTH as u16);
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

    // ld a, [wCurPartySpecies]
    cpu.a = cpu.borrow_wram().cur_party_species().map_or(0, Into::into);
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

    // ld de, sBoxMonNicknames
    cpu.set_de(sram::BOX_MON_NICKNAMES.1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld hl, wStringBuffer1
    cpu.set_hl(wram::STRING_BUFFER_1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, MON_NAME_LENGTH
    cpu.set_bc(MON_NAME_LENGTH as u16);
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

    // ld hl, wEnemyMon
    cpu.set_hl(0xd206); // wEnemyMon
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, sBoxMon1
    cpu.set_de(0xad26); // sBoxMon1
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, 1 + 1 + NUM_MOVES ; species + item + moves
    cpu.set_bc(1 + 1 + NUM_MOVES as u16); // species + item + moves
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

    // ld hl, wPlayerID
    cpu.set_hl(0xd47b); // wPlayerID
    cpu.pc += 3;
    cpu.cycle(12);

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // push de
    cpu.stack_push(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);

    // ld a, [wCurPartyLevel]
    cpu.a = cpu.borrow_wram().cur_party_level();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld d, a
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // callfar CalcExpAtLevel
    macros::farcall::callfar(cpu, 0x14, 0x4e47);

    // pop de
    {
        let de = cpu.stack_pop();
        cpu.set_de(de);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ldh a, [hProduct + 1]
    cpu.a = cpu.read_byte(hram::PRODUCT + 1);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ldh a, [hProduct + 2]
    cpu.a = cpu.read_byte(hram::PRODUCT + 2);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ldh a, [hProduct + 3]
    cpu.a = cpu.read_byte(hram::PRODUCT + 3);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // Set all 5 Experience Values to 0
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld b, 2 * NUM_EXP_STATS
    cpu.b = 2 * NUM_EXP_STATS;
    cpu.pc += 2;
    cpu.cycle(8);

    send_mon_into_box_loop2(cpu);
}

fn send_mon_into_box_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x5ee5;

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // dec b
    cpu.set_flag(CpuFlag::H, (cpu.b & 0x0f) == 0x00);
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .loop2
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return send_mon_into_box_loop2(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld hl, wEnemyMonDVs
    cpu.set_hl(0xd20c); // wEnemyMonDVs
    cpu.pc += 3;
    cpu.cycle(12);

    // ld b, 2 + NUM_MOVES ; DVs and PP ; wEnemyMonHappiness - wEnemyMonDVs
    cpu.b = 2 + NUM_MOVES; // DVs and PP ; wEnemyMonHappiness - wEnemyMonDVs
    cpu.pc += 2;
    cpu.cycle(8);

    send_mon_into_box_loop3(cpu);
}

fn send_mon_into_box_loop3(cpu: &mut Cpu) {
    cpu.pc = 0x5eef;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // dec b
    cpu.set_flag(CpuFlag::H, (cpu.b & 0x0f) == 0x00);
    cpu.b = cpu.b.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.b == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .loop3
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return send_mon_into_box_loop3(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, BASE_HAPPINESS
    cpu.a = BASE_HAPPINESS;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // inc de
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wCurPartyLevel]
    cpu.a = cpu.borrow_wram().cur_party_level();
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [de], a
    cpu.write_byte(cpu.de(), cpu.a);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wCurPartySpecies]
    cpu.a = cpu.borrow_wram().cur_party_species().map_or(0, Into::into);
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

    // ld a, [wCurPartySpecies]
    cpu.a = cpu.borrow_wram().cur_party_species().map_or(0, Into::into);
    cpu.pc += 3;
    cpu.cycle(16);

    // cp UNOWN
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(PokemonSpecies::Unown));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(PokemonSpecies::Unown) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(PokemonSpecies::Unown));
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .not_unown
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return send_mon_into_box_not_unown(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld hl, sBoxMon1DVs
    cpu.set_hl(0xad3b); // sBoxMon1DVs
    cpu.pc += 3;
    cpu.cycle(12);

    // predef GetUnownLetter
    macros::predef::predef_call!(cpu, GetUnownLetter);

    // callfar UpdateUnownDex
    macros::farcall::callfar(cpu, 0x3e, 0x7a18);

    send_mon_into_box_not_unown(cpu);
}

fn send_mon_into_box_not_unown(cpu: &mut Cpu) {
    cpu.pc = 0x5f20;

    // ld hl, sBoxMon1Moves
    cpu.set_hl(0xad28); // sBoxMon1Moves
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wTempMonMoves
    cpu.set_de(0xd110); // wTempMonMoves
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, NUM_MOVES
    cpu.set_bc(NUM_MOVES as u16);
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

    // ld hl, sBoxMon1PP
    cpu.set_hl(0xad3d); // sBoxMon1PP
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wTempMonPP
    cpu.set_de(0xd125); // wTempMonPP
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, NUM_MOVES
    cpu.set_bc(NUM_MOVES as u16);
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

    // ld b, 0
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // call RestorePPOfDepositedPokemon
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x5cb6); // RestorePPOfDepositedPokemon
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

    // scf
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::C, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn send_mon_into_box_full(cpu: &mut Cpu) {
    cpu.pc = 0x5f42;

    // call CloseSRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2fe1); // CloseSRAM
        cpu.pc = pc;
    }

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
