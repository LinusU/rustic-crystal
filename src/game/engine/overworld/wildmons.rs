use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            pokemon_data_constants::{GRASS_WILDDATA_LENGTH, NUM_GRASSMON},
            text_constants::MON_NAME_LENGTH,
        },
        data::wild::{johto_grass::JOHTO_GRASS_WILD_MONS, kanto_grass::KANTO_GRASS_WILD_MONS},
        macros,
        ram::wram,
    },
};

pub fn random_phone_wild_mon(cpu: &mut Cpu) {
    log::debug!("random_phone_wild_mon()");

    macros::farcall::farcall(cpu, 0x24, 0x4439); // GetCallerLocation

    log::trace!(
        "Caller location: group = {:#04x}, map = {:#04x}",
        cpu.b,
        cpu.c
    );

    cpu.d = cpu.b;
    cpu.e = cpu.c;

    cpu.set_hl(JOHTO_GRASS_WILD_MONS);
    cpu.set_bc(GRASS_WILDDATA_LENGTH as u16);
    cpu.call(0x6288); // LookUpWildmonsForMapDE

    if !cpu.flag(CpuFlag::C) {
        cpu.set_hl(KANTO_GRASS_WILD_MONS);
        cpu.call(0x6288); // LookUpWildmonsForMapDE
    }

    let time_of_day = cpu.borrow_wram().time_of_day();

    cpu.call(0x2f8c); // Random
    cpu.a &= 0b11;
    let pokemon_idx = cpu.a;

    cpu.set_hl(cpu.hl() + 5); // Skip header
    cpu.set_hl(cpu.hl() + u8::from(time_of_day) as u16 * NUM_GRASSMON as u16 * 2); // Skip to the correct time of day
    cpu.set_hl(cpu.hl() + pokemon_idx as u16 * 2); // Skip to the correct pokemon
    cpu.set_hl(cpu.hl() + 1); // Skip level

    let species = cpu.read_byte(cpu.hl());

    cpu.borrow_wram_mut().set_named_object_index(species);
    cpu.call(0x343b); // GetPokemonName

    cpu.set_hl(wram::STRING_BUFFER_1);
    cpu.set_de(wram::STRING_BUFFER_4);
    cpu.set_bc(MON_NAME_LENGTH as u16);
    cpu.call(0x3026); // CopyBytes

    cpu.pc = cpu.stack_pop(); // ret
}
