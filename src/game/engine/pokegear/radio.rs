use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            pokemon_data_constants::{GRASS_WILDDATA_LENGTH, NUM_GRASSMON},
            radio_constants::RadioChannelId,
            text_constants::MON_NAME_LENGTH,
        },
        data::wild::johto_grass::JOHTO_GRASS_WILD_MONS,
        macros,
        ram::wram,
    },
};

const OAKS_PKMNTALK_ROUTES: u16 = 0x47f2;
const OAKS_PKMNTALK_ROUTES_END: u16 = 0x4810;
const OAKS_PKMNTALK_ROUTES_LENGTH: u8 = (OAKS_PKMNTALK_ROUTES_END - OAKS_PKMNTALK_ROUTES) as u8;

pub fn oaks_pkmn_talk_4(cpu: &mut Cpu) {
    cpu.pc = 0x4762;

    // Choose a random route, and a random Pokemon from that route.
    oaks_pkmn_talk_4_sample(cpu);
}

fn oaks_pkmn_talk_4_sample(cpu: &mut Cpu) {
    cpu.pc = 0x4762;

    // call Random
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2f8c); // Random
        cpu.pc = pc;
    }

    // and %11111
    cpu.a &= 0b11111;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (0b11111 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0b11111);
    cpu.pc += 2;
    cpu.cycle(8);

    // cp (OaksPKMNTalkRoutes.End - OaksPKMNTalkRoutes) / 2
    cpu.set_flag(CpuFlag::Z, cpu.a == OAKS_PKMNTALK_ROUTES_LENGTH / 2);
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < ((OAKS_PKMNTALK_ROUTES_LENGTH / 2) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < OAKS_PKMNTALK_ROUTES_LENGTH / 2);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nc, .sample
    if !cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return oaks_pkmn_talk_4_sample(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld hl, OaksPKMNTalkRoutes
    cpu.set_hl(OAKS_PKMNTALK_ROUTES);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld c, a
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld b, 0
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

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

    // bc now contains the chosen map's group and number indices.
    // push bc
    cpu.stack_push(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);

    // Search the JohtoGrassWildMons array for the chosen map.
    // ld hl, JohtoGrassWildMons
    cpu.set_hl(JOHTO_GRASS_WILD_MONS);
    cpu.pc += 3;
    cpu.cycle(12);

    oaks_pkmn_talk_4_loop(cpu);
}

fn oaks_pkmn_talk_4_loop(cpu: &mut Cpu) {
    cpu.pc = 0x477a;

    // ld a, BANK(JohtoGrassWildMons)
    cpu.a = 0x0a; // BANK(JohtoGrassWildMons)
    cpu.pc += 2;
    cpu.cycle(8);

    // call GetFarByte
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x304d); // GetFarByte
        cpu.pc = pc;
    }

    // cp -1
    cpu.set_flag(CpuFlag::Z, cpu.a == 0xff);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < 0x0f);
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0xff);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .overflow
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return oaks_pkmn_talk_4_overflow(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // cp b
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.b);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.b & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.b);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .next
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return oaks_pkmn_talk_4_next(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, BANK(JohtoGrassWildMons)
    cpu.a = 0x0a; // BANK(JohtoGrassWildMons)
    cpu.pc += 2;
    cpu.cycle(8);

    // call GetFarByte
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x304d); // GetFarByte
        cpu.pc = pc;
    }

    // cp c
    cpu.set_flag(CpuFlag::Z, cpu.a == cpu.c);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (cpu.c & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < cpu.c);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .done
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return oaks_pkmn_talk_4_done(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    oaks_pkmn_talk_4_next(cpu);
}

fn oaks_pkmn_talk_4_next(cpu: &mut Cpu) {
    cpu.pc = 0x478f;

    // dec hl
    cpu.set_hl(cpu.hl().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld de, GRASS_WILDDATA_LENGTH
    cpu.set_de(GRASS_WILDDATA_LENGTH as u16);
    cpu.pc += 3;
    cpu.cycle(12);

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

    // jr .loop
    cpu.cycle(12);
    oaks_pkmn_talk_4_loop(cpu)
}

fn oaks_pkmn_talk_4_done(cpu: &mut Cpu) {
    cpu.pc = 0x4796;

    // Point hl to the list of morning Pokémon., skipping percentages
    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // Generate a number, either 0, 1, or 2, to choose a time of day.
    oaks_pkmn_talk_4_loop2(cpu);
}

fn oaks_pkmn_talk_4_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x479a;

    // call Random
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2f8c); // Random
        cpu.pc = pc;
    }

    // maskbits NUM_DAYTIMES
    // and 0b11
    cpu.a &= 0b11;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (0b11 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0b11);
    cpu.pc += 2;
    cpu.cycle(8);

    // cp DARKNESS_F
    cpu.set_flag(CpuFlag::Z, cpu.a == 3);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (3 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 3);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .loop2
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return oaks_pkmn_talk_4_loop2(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld bc, 2 * NUM_GRASSMON
    cpu.set_bc(2 * NUM_GRASSMON as u16);

    // call AddNTimes
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x30fe); // AddNTimes
        cpu.pc = pc;
    }

    oaks_pkmn_talk_4_loop3(cpu);
}

fn oaks_pkmn_talk_4_loop3(cpu: &mut Cpu) {
    cpu.pc = 0x47a9;

    // Choose one of the middle three Pokemon.
    // call Random
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2f8c); // Random
        cpu.pc = pc;
    }

    // maskbits NUM_GRASSMON
    // and 0b111
    cpu.a &= 0b111;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (0b111 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 0b111);
    cpu.pc += 2;
    cpu.cycle(8);

    // cp 2
    cpu.set_flag(CpuFlag::Z, cpu.a == 2);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (2 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 2);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr c, .loop3
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return oaks_pkmn_talk_4_loop3(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // cp 5
    cpu.set_flag(CpuFlag::Z, cpu.a == 5);
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (5 & 0x0f));
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < 5);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nc, .loop3
    if !cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return oaks_pkmn_talk_4_loop3(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld e, a
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // ld d, 0
    cpu.d = 0;
    cpu.pc += 2;
    cpu.cycle(8);

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

    // inc hl ; skip level
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, BANK(JohtoGrassWildMons)
    cpu.a = 0x0a; // BANK(JohtoGrassWildMons)
    cpu.pc += 2;
    cpu.cycle(8);

    // call GetFarByte
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x304d); // GetFarByte
        cpu.pc = pc;
    }

    // ld [wNamedObjectIndex], a
    let named_object_index = cpu.a;
    cpu.borrow_wram_mut()
        .set_named_object_index(named_object_index);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [wCurPartySpecies], a
    let cur_party_species = cpu.a;
    cpu.borrow_wram_mut()
        .set_cur_party_species(Some(cur_party_species.into()));
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

    // ld hl, wStringBuffer1
    cpu.set_hl(wram::STRING_BUFFER_1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld de, wMonOrItemNameBuffer
    cpu.set_de(wram::MON_OR_ITEM_NAME_BUFFER);
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

    // Now that we've chosen our wild Pokemon,
    // let's recover the map index info and get its name.
    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // call GetWorldMapLocation
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2caf); // GetWorldMapLocation
        cpu.pc = pc;
    }

    // ld e, a
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    // farcall GetLandmarkName
    macros::farcall::farcall(cpu, 0x72, 0x68a5);

    // ld hl, OPT_OakText1
    cpu.set_hl(0x482f); // OPT_OakText1
    cpu.pc += 3;
    cpu.cycle(12);

    // call CopyRadioTextToRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x51dc); // CopyRadioTextToRAM
        cpu.pc = pc;
    }

    // ld a, OAKS_POKEMON_TALK_5
    cpu.a = RadioChannelId::OaksPokemonTalk5.into();
    cpu.pc += 2;
    cpu.cycle(8);

    // jp PrintRadioLine
    cpu.cycle(16);
    cpu.jump(0x46ea); // PrintRadioLine
}

fn oaks_pkmn_talk_4_overflow(cpu: &mut Cpu) {
    cpu.pc = 0x47ec;

    // pop bc
    {
        let bc = cpu.stack_pop();
        cpu.set_bc(bc);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ld a, OAKS_POKEMON_TALK
    cpu.a = RadioChannelId::OaksPokemonTalk.into();
    cpu.pc += 2;
    cpu.cycle(8);

    // jp PrintRadioLine
    cpu.cycle(16);
    cpu.jump(0x46ea); // PrintRadioLine
}
