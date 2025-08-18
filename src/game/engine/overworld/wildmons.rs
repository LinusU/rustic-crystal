use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            battle_constants::BattleType, gfx_constants, landmark_constants::Region,
            map_constants::Map, pokemon_constants::PokemonSpecies, ram_constants::SwarmFlags,
            text_constants::MON_NAME_LENGTH,
        },
        data::wild::{
            johto_grass::JOHTO_GRASS_WILD_MONS,
            johto_water::JOHTO_WATER_WILD_MONS,
            kanto_grass::KANTO_GRASS_WILD_MONS,
            kanto_water::KANTO_WATER_WILD_MONS,
            probabilities::{GRASS_MON_PROB_TABLE, WATER_MON_PROB_TABLE},
            swarm_grass::SWARM_GRASS_WILD_MONS,
            swarm_water::SWARM_WATER_WILD_MONS,
        },
        macros::{
            self,
            asserts::{GrassWildmons, WaterWildmons, Wildmons},
        },
        ram::wram,
    },
};

pub fn load_wild_mon_data(cpu: &mut Cpu) {
    log::debug!("load_wild_mon_data()");

    let grass = grass_wildmon_lookup(cpu).map_or([0, 0, 0], |g| g.encounter_rates);
    let water = water_wildmon_lookup(cpu).map_or(0, |w| w.encounter_rate);

    cpu.borrow_wram_mut().set_morn_encounter_rate(grass[0]);
    cpu.borrow_wram_mut().set_day_encounter_rate(grass[1]);
    cpu.borrow_wram_mut().set_nite_encounter_rate(grass[2]);
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

fn find_nest_find_grass(
    cpu: &mut Cpu,
    species: PokemonSpecies,
    grass: &'static [(Map, GrassWildmons)],
) {
    for (map, wildmons) in grass {
        find_nest_search_map_for_mon(cpu, species, *map, Wildmons::Grass(wildmons));
    }
}

fn find_nest_find_water(
    cpu: &mut Cpu,
    species: PokemonSpecies,
    water: &'static [(Map, WaterWildmons)],
) {
    for (map, wildmons) in water {
        find_nest_search_map_for_mon(cpu, species, *map, Wildmons::Water(wildmons));
    }
}

fn find_nest_search_map_for_mon(cpu: &mut Cpu, species: PokemonSpecies, map: Map, mons: Wildmons) {
    for (_, encounter) in mons.all_encounters() {
        if encounter == species {
            find_nest_append_nest(cpu, map);
        }
    }
}

fn find_nest_roam_mon_1(cpu: &mut Cpu, species: PokemonSpecies) {
    if cpu.borrow_wram().roam_mon_1_species() == Some(species) {
        find_nest_append_nest(cpu, cpu.borrow_wram().roam_mon_1_map())
    }
}

fn find_nest_roam_mon_2(cpu: &mut Cpu, species: PokemonSpecies) {
    if cpu.borrow_wram().roam_mon_2_species() == Some(species) {
        find_nest_append_nest(cpu, cpu.borrow_wram().roam_mon_2_map())
    }
}

fn find_nest_append_nest(cpu: &mut Cpu, map: Map) {
    (cpu.b, cpu.c) = map.into();
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

pub fn choose_wild_encounter(cpu: &mut Cpu) {
    fn return_value(cpu: &mut Cpu, value: bool) {
        cpu.set_flag(CpuFlag::Z, value);
        cpu.set_flag(CpuFlag::C, false);
        cpu.pc = cpu.stack_pop(); // ret
    }

    let Some(wildmons) = load_wild_mon_data_pointer(cpu) else {
        return return_value(cpu, false);
    };

    if check_encounter_roam_mon(cpu) {
        return return_value(cpu, true);
    }

    let encounters = match wildmons {
        Wildmons::Grass(data) => data.encounters(cpu.borrow_wram().time_of_day()),
        Wildmons::Water(data) => &data.encounters,
    };

    let prob_table = match wildmons {
        Wildmons::Grass(_) => GRASS_MON_PROB_TABLE,
        Wildmons::Water(_) => WATER_MON_PROB_TABLE,
    };

    let rng = loop {
        cpu.call(0x2f8c); // Random

        if cpu.a < 100 {
            break cpu.a + 1;
        }
    };

    let index = prob_table
        .iter()
        .position(|&threshold| rng <= threshold)
        .expect("No valid mon found for this RNG value");

    // this selects our mon
    let mut level = encounters[index].0;
    let species = encounters[index].1;

    if species == PokemonSpecies::Unown && cpu.borrow_wram().unlocked_unowns().is_empty() {
        return return_value(cpu, false);
    }

    // If the Pokemon is encountered by surfing, we need to give the levels some variety.
    if matches!(wildmons, Wildmons::Water(_)) {
        // Check if we buff the wild mon, and by how much.
        cpu.call(0x2f8c); // Random

        match cpu.a {
            0..=89 => {}             // ~35% chance
            90..=165 => level += 1,  // ~30% chance
            166..=216 => level += 2, // ~20% chance
            217..=242 => level += 3, // ~10% chance
            243..=255 => level += 4, //  ~5% chance
        }
    }

    cpu.borrow_wram_mut().set_cur_party_level(level);

    cpu.borrow_wram_mut()
        .set_temp_wild_mon_species(Some(species));

    return_value(cpu, true)
}

fn load_wild_mon_data_pointer(cpu: &mut Cpu) -> Option<Wildmons> {
    cpu.call(0x1852); // CheckOnWater

    if cpu.flag(CpuFlag::Z) {
        water_wildmon_lookup(cpu).map(Wildmons::Water)
    } else {
        grass_wildmon_lookup(cpu).map(Wildmons::Grass)
    }
}

fn grass_wildmon_lookup(cpu: &mut Cpu) -> Option<&'static GrassWildmons> {
    swarm_wildmon_check(cpu, SWARM_GRASS_WILD_MONS).or_else(|| {
        let wild_data = johto_wildmon_check(cpu, JOHTO_GRASS_WILD_MONS, KANTO_GRASS_WILD_MONS);
        normal_wildmon_ok(cpu, wild_data)
    })
}

fn water_wildmon_lookup(cpu: &mut Cpu) -> Option<&'static WaterWildmons> {
    swarm_wildmon_check(cpu, SWARM_WATER_WILD_MONS).or_else(|| {
        let wild_data = johto_wildmon_check(cpu, JOHTO_WATER_WILD_MONS, KANTO_WATER_WILD_MONS);
        normal_wildmon_ok(cpu, wild_data)
    })
}

fn johto_wildmon_check<T>(cpu: &mut Cpu, johto: T, kanto: T) -> T {
    cpu.call(0x2f17); // IsInJohto

    if Region::from(cpu.a) == Region::Johto {
        johto
    } else {
        kanto
    }
}

fn swarm_wildmon_check<'a, T>(cpu: &mut Cpu, wildmons: &'a [(Map, T)]) -> Option<&'a T> {
    let map = cpu.borrow_wram().map();
    let swarm_flags = cpu.borrow_wram().swarm_flags();

    if swarm_flags.contains(SwarmFlags::DUNSPARCE_SWARM) && cpu.borrow_wram().dunsparce_map() == map
    {
        return look_up_wildmons_for_map(map, wildmons);
    }

    if swarm_flags.contains(SwarmFlags::YANMA_SWARM) && cpu.borrow_wram().yanma_map() == map {
        return look_up_wildmons_for_map(map, wildmons);
    }

    None
}

fn normal_wildmon_ok<'a, T>(cpu: &mut Cpu, wildmons: &'a [(Map, T)]) -> Option<&'a T> {
    look_up_wildmons_for_map(cpu.borrow_wram().map(), wildmons)
}

pub fn copy_curr_map_de(cpu: &mut Cpu) {
    (cpu.d, cpu.e) = cpu.borrow_wram().map().into();
    cpu.pc = cpu.stack_pop(); // ret
}

fn look_up_wildmons_for_map<T>(map: Map, wildmons: &[(Map, T)]) -> Option<&T> {
    wildmons
        .iter()
        .find(|(key, _)| *key == map)
        .map(|(_, data)| data)
}

fn check_encounter_roam_mon(cpu: &mut Cpu) -> bool {
    // Don't trigger an encounter if we're on water.
    cpu.call(0x1852); // CheckOnWater

    if cpu.flag(CpuFlag::Z) {
        return false;
    }

    // Randomly select a beast.
    cpu.call(0x2f8c); // Random

    // 25/64 chance
    if cpu.a >= 100 {
        return false;
    }

    // Of that, a 3/4 chance.  Running total: 75/256, or around 29.3%.
    cpu.a &= 0b00000011;

    if cpu.a == 0 {
        return false;
    }

    // 1/3 chance that it's Entei, 1/3 chance that it's Raikou
    let roam_mon = cpu.a;

    // Compare its current location with yours

    let roam_mon_map = match roam_mon {
        1 => cpu.borrow_wram().roam_mon_1_map(),
        2 => cpu.borrow_wram().roam_mon_2_map(),
        3 => cpu.borrow_wram().roam_mon_3_map(),
        n => panic!("Invalid roam mon index: {n}"),
    };

    if roam_mon_map != cpu.borrow_wram().map() {
        return false;
    }

    // We've decided to take on a beast, so stage its information for battle.

    let species = match roam_mon {
        1 => cpu.borrow_wram().roam_mon_1_species(),
        2 => cpu.borrow_wram().roam_mon_2_species(),
        3 => cpu.borrow_wram().roam_mon_3_species(),
        n => panic!("Invalid roam mon index: {n}"),
    };

    let level = match roam_mon {
        1 => cpu.borrow_wram().roam_mon_1_level(),
        2 => cpu.borrow_wram().roam_mon_2_level(),
        3 => cpu.borrow_wram().roam_mon_3_level(),
        n => panic!("Invalid roam mon index: {n}"),
    };

    cpu.borrow_wram_mut().set_temp_wild_mon_species(species);
    cpu.borrow_wram_mut().set_cur_party_level(level);
    cpu.borrow_wram_mut().set_battle_type(BattleType::Roaming);

    true
}

/// Finds a rare wild Pokemon in the route of the trainer calling, then checks if it's been Seen already.
/// The trainer will then tell you about the Pokemon if you haven't seen it.
pub fn random_unseen_wild_mon(cpu: &mut Cpu) {
    fn return_value(cpu: &mut Cpu, value: u8) {
        cpu.borrow_wram_mut().set_script_var(value);
        cpu.a = value;
        cpu.pc = cpu.stack_pop(); // ret
    }

    log::debug!("random_unseen_wild_mon()");

    macros::farcall::farcall(cpu, 0x24, 0x4439); // GetCallerLocation
    let map = Map::from((cpu.b, cpu.c));

    log::trace!("Caller location: {map:?}");

    let wildmons = look_up_wildmons_for_map(map, JOHTO_GRASS_WILD_MONS)
        .or(look_up_wildmons_for_map(map, KANTO_GRASS_WILD_MONS));

    let Some(wildmons) = wildmons else {
        log::warn!("No matching wildmons found for map {map:?}");
        return return_value(cpu, 1);
    };

    let encounters = wildmons.encounters(cpu.borrow_wram().time_of_day());

    let pokemon_idx = loop {
        cpu.call(0x2f8c); // Random
        cpu.a &= 0b11;

        if cpu.a != 0 {
            break (4 + cpu.a - 1) as usize; // Random int 4..=6
        }
    };

    let possibly_rare_species = encounters[pokemon_idx].1;

    // Check that the possibly rare species is actually rare.
    for encounter in encounters.iter().take(4) {
        if possibly_rare_species == encounter.1 {
            return return_value(cpu, 1);
        }
    }

    cpu.a = u8::from(possibly_rare_species) - 1;
    cpu.call(0x339b); // CheckSeenMon
    let is_seen = !cpu.flag(CpuFlag::Z);

    if is_seen {
        return return_value(cpu, 1);
    }

    // Since we haven't seen it, have the caller tell us about it.
    cpu.set_de(wram::STRING_BUFFER_1);
    cpu.call(0x30d6); // CopyName1

    cpu.borrow_wram_mut()
        .set_named_object_index(possibly_rare_species.into());
    cpu.call(0x343b); // GetPokemonName

    cpu.set_hl(0x651a); // RandomUnseenWildMon.JustSawSomeRareMonText
    cpu.call(0x1057); // PrintText

    return_value(cpu, 0)
}

pub fn random_phone_wild_mon(cpu: &mut Cpu) {
    log::debug!("random_phone_wild_mon()");

    macros::farcall::farcall(cpu, 0x24, 0x4439); // GetCallerLocation
    let map = Map::from((cpu.b, cpu.c));

    log::trace!("Caller location: {map:?}");

    let wildmons = look_up_wildmons_for_map(map, JOHTO_GRASS_WILD_MONS)
        .or(look_up_wildmons_for_map(map, KANTO_GRASS_WILD_MONS))
        .unwrap_or_else(|| panic!("No matching wildmons found for map {map:?}"));

    cpu.call(0x2f8c); // Random
    cpu.a &= 0b11;
    let pokemon_idx = cpu.a as usize;

    let species = wildmons.encounters(cpu.borrow_wram().time_of_day())[pokemon_idx].1;

    cpu.borrow_wram_mut().set_named_object_index(species.into());
    cpu.call(0x343b); // GetPokemonName

    cpu.set_hl(wram::STRING_BUFFER_1);
    cpu.set_de(wram::STRING_BUFFER_4);
    cpu.set_bc(MON_NAME_LENGTH as u16);
    cpu.call(0x3026); // CopyBytes

    cpu.pc = cpu.stack_pop(); // ret
}
