use crate::{
    cpu::Cpu,
    game::{
        constants::{
            radio_constants::RadioChannelId, ram_constants::TimeOfDay,
            text_constants::MON_NAME_LENGTH,
        },
        data::{
            radio::oaks_pkmn_talk_routes::OAKS_PKMN_TALK_ROUTES,
            wild::johto_grass::JOHTO_GRASS_WILD_MONS,
        },
        macros,
        ram::wram,
    },
};

pub fn oaks_pkmn_talk_4(cpu: &mut Cpu) {
    log::debug!("oaks_pkmn_talk_4()");

    // Choose a random route
    let route_idx = loop {
        cpu.call(0x2f8c); // Random
        cpu.a &= 0b11111;

        if cpu.a < OAKS_PKMN_TALK_ROUTES.len() as u8 {
            break cpu.a;
        }
    };

    let map = OAKS_PKMN_TALK_ROUTES[route_idx as usize];

    let wildmons = JOHTO_GRASS_WILD_MONS
        .iter()
        .find(|(m, _)| *m == map)
        .map(|(_, d)| d);

    let Some(wildmons) = wildmons else {
        log::warn!("No JohtoGrassWildMons found for map {map:?}");
        cpu.a = RadioChannelId::OaksPokemonTalk.into();
        return cpu.jump(0x46ea); // PrintRadioLine
    };

    // Generate a random time of day.
    let time_of_day = loop {
        cpu.call(0x2f8c); // Random

        match cpu.a & 0b11 {
            0b00 => break TimeOfDay::Morn,
            0b01 => break TimeOfDay::Day,
            0b10 => break TimeOfDay::Nite,
            _ => continue,
        }
    };

    // Choose one of the middle three Pokemon.
    let pokemon_idx = loop {
        cpu.call(0x2f8c); // Random
        cpu.a &= 0b111;

        if cpu.a >= 2 && cpu.a < 5 {
            break cpu.a as usize;
        }
    };

    let species = wildmons.encounters(time_of_day)[pokemon_idx].1;

    cpu.borrow_wram_mut().set_named_object_index(species.into());
    cpu.borrow_wram_mut().set_cur_party_species(Some(species));
    cpu.call(0x343b); // GetPokemonName

    cpu.set_hl(wram::STRING_BUFFER_1);
    cpu.set_de(wram::MON_OR_ITEM_NAME_BUFFER);
    cpu.set_bc(MON_NAME_LENGTH as u16);
    cpu.call(0x3026); // CopyBytes

    (cpu.b, cpu.c) = map.into();
    cpu.call(0x2caf); // GetWorldMapLocation
    let landmark_id = cpu.a;

    cpu.e = landmark_id;
    macros::farcall::farcall(cpu, 0x72, 0x68a5); // GetLandmarkName

    cpu.set_hl(0x482f); // OPT_OakText1
    cpu.call(0x51dc); // CopyRadioTextToRAM

    cpu.a = RadioChannelId::OaksPokemonTalk5.into();
    cpu.jump(0x46ea); // PrintRadioLine
}
