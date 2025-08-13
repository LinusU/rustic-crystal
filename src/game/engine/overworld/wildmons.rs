use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            gfx_constants,
            landmark_constants::Region,
            pokemon_constants::PokemonSpecies,
            pokemon_data_constants::{
                GRASS_WILDDATA_LENGTH, NUM_GRASSMON, NUM_WATERMON, WATER_WILDDATA_LENGTH,
            },
            text_constants::MON_NAME_LENGTH,
        },
        data::wild::{
            johto_grass::JOHTO_GRASS_WILD_MONS, johto_water::JOHTO_WATER_WILD_MONS,
            kanto_grass::KANTO_GRASS_WILD_MONS, kanto_water::KANTO_WATER_WILD_MONS,
        },
        macros,
        ram::wram,
    },
};

pub fn load_wild_mon_data(cpu: &mut Cpu) {
    log::debug!("load_wild_mon_data()");

    cpu.call(0x6205); // _GrassWildmonLookup

    let grass = if cpu.flag(CpuFlag::C) {
        let morn = cpu.read_byte(cpu.hl() + 2);
        let day = cpu.read_byte(cpu.hl() + 3);
        let nite = cpu.read_byte(cpu.hl() + 4);
        (morn, day, nite)
    } else {
        (0, 0, 0)
    };

    cpu.call(0x621d); // _WaterWildmonLookup

    let water = if cpu.flag(CpuFlag::C) {
        cpu.set_hl(cpu.hl() + 2);
        cpu.read_byte(cpu.hl())
    } else {
        0
    };

    cpu.borrow_wram_mut().set_morn_encounter_rate(grass.0);
    cpu.borrow_wram_mut().set_day_encounter_rate(grass.1);
    cpu.borrow_wram_mut().set_nite_encounter_rate(grass.2);
    cpu.borrow_wram_mut().set_water_encounter_rate(water);

    cpu.pc = cpu.stack_pop(); // ret
}

pub fn find_nest(cpu: &mut Cpu) {
    let region = Region::from(cpu.e);
    let species = PokemonSpecies::from(cpu.borrow_wram().named_object_index());

    log::info!("find_nest({region:?}, {species:?})");

    cpu.a = 0;
    cpu.set_hl(macros::coords::coord!(0, 0));
    cpu.set_bc(gfx_constants::SCREEN_WIDTH as u16 * gfx_constants::SCREEN_HEIGHT as u16);
    cpu.call(0x3041); // ByteFill

    // Start of array to fill up with nests
    cpu.set_de(macros::coords::coord!(0, 0));

    match region {
        Region::Johto => {
            find_nest_find_grass(cpu, species, JOHTO_GRASS_WILD_MONS);
            find_nest_find_water(cpu, species, JOHTO_WATER_WILD_MONS);

            find_nest_roam_mon_1(cpu, species);
            find_nest_roam_mon_2(cpu, species);
        }

        // Kanto
        _ => {
            find_nest_find_grass(cpu, species, KANTO_GRASS_WILD_MONS);
            find_nest_find_water(cpu, species, KANTO_WATER_WILD_MONS);
        }
    }

    cpu.pc = cpu.stack_pop(); // ret
}

fn find_nest_find_grass(cpu: &mut Cpu, species: PokemonSpecies, mons_addr: u16) {
    for i in 0.. {
        let base = mons_addr + i * GRASS_WILDDATA_LENGTH as u16;

        if cpu.read_byte(base) == 0xff {
            break;
        }

        let map_group = cpu.read_byte(base);
        let map_id = cpu.read_byte(base + 1);

        find_nest_search_map_for_mon(cpu, species, map_group, map_id, base + 5, NUM_GRASSMON * 3);
    }
}

fn find_nest_find_water(cpu: &mut Cpu, species: PokemonSpecies, mons_addr: u16) {
    for i in 0.. {
        let base = mons_addr + i * WATER_WILDDATA_LENGTH as u16;

        if cpu.read_byte(base) == 0xff {
            break;
        }

        let map_group = cpu.read_byte(base);
        let map_id = cpu.read_byte(base + 1);

        find_nest_search_map_for_mon(cpu, species, map_group, map_id, base + 3, NUM_WATERMON);
    }
}

fn find_nest_search_map_for_mon(
    cpu: &mut Cpu,
    species: PokemonSpecies,
    map_group: u8,
    map_id: u8,
    addr: u16,
    size: usize,
) {
    for i in 0..(size as u16) {
        if PokemonSpecies::from(cpu.read_byte(addr + 1 + i * 2)) == species {
            return find_nest_append_nest(cpu, map_group, map_id);
        }
    }
}

fn find_nest_roam_mon_1(cpu: &mut Cpu, species: PokemonSpecies) {
    if cpu.borrow_wram().roam_mon_1_species() == Some(species) {
        let map_group = cpu.borrow_wram().roam_mon_1_map_group();
        let map_id = cpu.borrow_wram().roam_mon_1_map_number();

        find_nest_append_nest(cpu, map_group, map_id)
    }
}

fn find_nest_roam_mon_2(cpu: &mut Cpu, species: PokemonSpecies) {
    if cpu.borrow_wram().roam_mon_2_species() == Some(species) {
        let map_group = cpu.borrow_wram().roam_mon_2_map_group();
        let map_id = cpu.borrow_wram().roam_mon_2_map_number();

        find_nest_append_nest(cpu, map_group, map_id)
    }
}

fn find_nest_append_nest(cpu: &mut Cpu, map_group: u8, map_id: u8) {
    cpu.b = map_group;
    cpu.c = map_id;
    cpu.call(0x2caf); // GetWorldMapLocation
    let pokegear_location = cpu.a;

    for i in 0..(gfx_constants::SCREEN_WIDTH as u16 * gfx_constants::SCREEN_HEIGHT as u16) {
        if cpu.read_byte(macros::coords::coord!(0, 0) + i) == pokegear_location {
            return; // Already found this location
        }
    }

    cpu.write_byte(cpu.de(), pokegear_location);
    cpu.set_de(cpu.de() + 1);
}

pub fn load_wild_mon_data_pointer(cpu: &mut Cpu) {
    cpu.call(0x1852); // CheckOnWater

    if cpu.flag(CpuFlag::Z) {
        cpu.jump(0x621d) // _WaterWildmonLookup
    } else {
        cpu.jump(0x6205) // _GrassWildmonLookup
    }
}

/// Finds a rare wild Pokemon in the route of the trainer calling, then checks if it's been Seen already.
/// The trainer will then tell you about the Pokemon if you haven't seen it.
pub fn random_unseen_wild_mon(cpu: &mut Cpu) {
    log::debug!("random_unseen_wild_mon()");

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

        if !cpu.flag(CpuFlag::C) {
            log::warn!(
                "No matching wildmons found for group_id {:#02x}, map_id {:#02x}",
                cpu.d,
                cpu.e
            );

            return random_unseen_wild_mon_done(cpu);
        }
    }

    let map_wildmons_addr = cpu.hl();

    let time_of_day = cpu.borrow_wram().time_of_day();

    let pokemon_idx = loop {
        cpu.call(0x2f8c); // Random
        cpu.a &= 0b11;

        if cpu.a != 0 {
            break 4 + cpu.a - 1; // Random int 4..=6
        }
    };

    cpu.set_hl(map_wildmons_addr + 5); // Skip header
    cpu.set_hl(cpu.hl() + u8::from(time_of_day) as u16 * NUM_GRASSMON as u16 * 2); // Skip to the correct time of day
    cpu.set_hl(cpu.hl() + pokemon_idx as u16 * 2); // Skip to the correct pokemon
    cpu.set_hl(cpu.hl() + 1); // Skip level
    let possibly_rare_species = cpu.read_byte(cpu.hl());

    // Species index of the most common Pokemon on that route

    for i in 0..4 {
        cpu.set_hl(map_wildmons_addr + 5); // Skip header
        cpu.set_hl(cpu.hl() + u8::from(time_of_day) as u16 * NUM_GRASSMON as u16 * 2); // Skip to the correct time of day
        cpu.set_hl(cpu.hl() + i * 2); // Skip to the correct pokemon
        cpu.set_hl(cpu.hl() + 1); // Skip level
        let common_species = cpu.read_byte(cpu.hl());

        if possibly_rare_species == common_species {
            return random_unseen_wild_mon_done(cpu);
        }
    }

    cpu.a = possibly_rare_species.wrapping_sub(1);
    cpu.call(0x339b); // CheckSeenMon
    let is_seen = !cpu.flag(CpuFlag::Z);

    if is_seen {
        return random_unseen_wild_mon_done(cpu);
    }

    // Since we haven't seen it, have the caller tell us about it.
    cpu.set_de(wram::STRING_BUFFER_1);
    cpu.call(0x30d6); // CopyName1

    cpu.borrow_wram_mut()
        .set_named_object_index(possibly_rare_species);
    cpu.call(0x343b); // GetPokemonName

    cpu.set_hl(0x651a); // RandomUnseenWildMon.JustSawSomeRareMonText
    cpu.call(0x1057); // PrintText

    cpu.borrow_wram_mut().set_script_var(0);
    cpu.a = 0;

    cpu.pc = cpu.stack_pop(); // ret
}

fn random_unseen_wild_mon_done(cpu: &mut Cpu) {
    cpu.borrow_wram_mut().set_script_var(1);
    cpu.a = 1;

    cpu.pc = cpu.stack_pop(); // ret
}

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
