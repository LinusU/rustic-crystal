use crate::{
    cpu::Cpu,
    game::{
        constants::{
            pokemon_constants::PokemonSpecies,
            pokemon_data_constants::{GRASS_WILDDATA_LENGTH, NUM_GRASSMON},
            radio_constants::RadioChannelId,
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

    let (group_id, map_id) = OAKS_PKMN_TALK_ROUTES[route_idx as usize];

    // Search the JohtoGrassWildMons array for the chosen map.
    for i in 0.. {
        cpu.a = 0x0a; // BANK(JohtoGrassWildMons)
        cpu.set_hl(JOHTO_GRASS_WILD_MONS + i * GRASS_WILDDATA_LENGTH as u16);
        cpu.call(0x304d); // GetFarByte

        if cpu.a == 0xff {
            log::warn!("No matching JohtoGrassWildMons found for group_id {group_id:#02x}, map_id {map_id:#02x}");
            cpu.a = RadioChannelId::OaksPokemonTalk.into();
            return cpu.jump(0x46ea); // PrintRadioLine
        }

        if cpu.a == group_id {
            cpu.a = 0x0a; // BANK(JohtoGrassWildMons)
            cpu.set_hl(cpu.hl() + 1);
            cpu.call(0x304d); // GetFarByte

            if cpu.a == map_id {
                break;
            }
        }
    }

    let johto_grass_wildmons_addr = cpu.hl();

    // Generate a number, either 0, 1, or 2, to choose a time of day.
    let time_of_day = loop {
        cpu.call(0x2f8c); // Random
        cpu.a &= 0b11;

        if cpu.a != 0b11 {
            break cpu.a;
        }
    };

    // Choose one of the middle three Pokemon.
    let pokemon_idx = loop {
        cpu.call(0x2f8c); // Random
        cpu.a &= 0b111;

        if cpu.a >= 2 && cpu.a < 5 {
            break cpu.a;
        }
    };

    cpu.set_hl(johto_grass_wildmons_addr + 4); // Skipping percentages
    cpu.set_hl(cpu.hl() + (time_of_day as u16 * 2 * NUM_GRASSMON as u16)); // Skip to the time of day
    cpu.set_hl(cpu.hl() + (pokemon_idx as u16 * 2)); // Skip to the chosen Pokémon
    cpu.set_hl(cpu.hl() + 1); // Skip level

    cpu.a = 0x0a; // BANK(JohtoGrassWildMons)
    cpu.call(0x304d); // GetFarByte
    let species = PokemonSpecies::from(cpu.a);

    cpu.borrow_wram_mut().set_named_object_index(species.into());
    cpu.borrow_wram_mut().set_cur_party_species(Some(species));
    cpu.call(0x343b); // GetPokemonName

    cpu.set_hl(wram::STRING_BUFFER_1);
    cpu.set_de(wram::MON_OR_ITEM_NAME_BUFFER);
    cpu.set_bc(MON_NAME_LENGTH as u16);
    cpu.call(0x3026); // CopyBytes

    cpu.b = group_id;
    cpu.c = map_id;
    cpu.call(0x2caf); // GetWorldMapLocation
    let landmark_id = cpu.a;

    cpu.e = landmark_id;
    macros::farcall::farcall(cpu, 0x72, 0x68a5); // GetLandmarkName

    cpu.set_hl(0x482f); // OPT_OakText1
    cpu.call(0x51dc); // CopyRadioTextToRAM

    cpu.a = RadioChannelId::OaksPokemonTalk5.into();
    cpu.jump(0x46ea); // PrintRadioLine
}
