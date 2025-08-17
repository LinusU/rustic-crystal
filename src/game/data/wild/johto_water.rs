use crate::game::{
    constants::{map_constants::Map, pokemon_constants::PokemonSpecies},
    macros::{asserts::WaterWildmons, data::percent},
};

pub const JOHTO_WATER_WILD_MONS: &[(Map, WaterWildmons)] = &[
    (
        Map::RuinsOfAlphOutside,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (15, PokemonSpecies::Wooper),
                (20, PokemonSpecies::Quagsire),
                (15, PokemonSpecies::Quagsire),
            ],
        },
    ),
    (
        Map::UnionCave1F,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (15, PokemonSpecies::Wooper),
                (20, PokemonSpecies::Quagsire),
                (15, PokemonSpecies::Quagsire),
            ],
        },
    ),
    (
        Map::UnionCaveB1F,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (15, PokemonSpecies::Wooper),
                (20, PokemonSpecies::Quagsire),
                (15, PokemonSpecies::Quagsire),
            ],
        },
    ),
    (
        Map::UnionCaveB2F,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (15, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Quagsire),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::SlowpokeWellB1F,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (15, PokemonSpecies::Slowpoke),
                (20, PokemonSpecies::Slowpoke),
                (10, PokemonSpecies::Slowpoke),
            ],
        },
    ),
    (
        Map::SlowpokeWellB2F,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (15, PokemonSpecies::Slowpoke),
                (20, PokemonSpecies::Slowpoke),
                (20, PokemonSpecies::Slowbro),
            ],
        },
    ),
    (
        Map::IlexForest,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (15, PokemonSpecies::Psyduck),
                (10, PokemonSpecies::Psyduck),
                (15, PokemonSpecies::Golduck),
            ],
        },
    ),
    (
        Map::MountMortar1FOutside,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (15, PokemonSpecies::Goldeen),
                (20, PokemonSpecies::Marill),
                (20, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::MountMortar2FInside,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (20, PokemonSpecies::Goldeen),
                (25, PokemonSpecies::Marill),
                (25, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::MountMortarB1F,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (15, PokemonSpecies::Goldeen),
                (20, PokemonSpecies::Marill),
                (20, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::WhirlIslandSw,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (20, PokemonSpecies::Tentacool),
                (15, PokemonSpecies::Horsea),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::WhirlIslandB2F,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (15, PokemonSpecies::Horsea),
                (20, PokemonSpecies::Horsea),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::WhirlIslandLugiaChamber,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (20, PokemonSpecies::Horsea),
                (20, PokemonSpecies::Tentacruel),
                (20, PokemonSpecies::Seadra),
            ],
        },
    ),
    (
        Map::SilverCaveRoom2,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (35, PokemonSpecies::Seaking),
                (35, PokemonSpecies::Golduck),
                (35, PokemonSpecies::Goldeen),
            ],
        },
    ),
    (
        Map::DarkCaveVioletEntrance,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (15, PokemonSpecies::Magikarp),
                (10, PokemonSpecies::Magikarp),
                (5, PokemonSpecies::Magikarp),
            ],
        },
    ),
    (
        Map::DarkCaveBlackthornEntrance,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (15, PokemonSpecies::Magikarp),
                (10, PokemonSpecies::Magikarp),
                (5, PokemonSpecies::Magikarp),
            ],
        },
    ),
    (
        Map::DragonsDenB1F,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (15, PokemonSpecies::Magikarp),
                (10, PokemonSpecies::Magikarp),
                (10, PokemonSpecies::Dratini),
            ],
        },
    ),
    (
        Map::OlivinePort,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (20, PokemonSpecies::Tentacool),
                (15, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route30,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (20, PokemonSpecies::Poliwag),
                (15, PokemonSpecies::Poliwag),
                (20, PokemonSpecies::Poliwhirl),
            ],
        },
    ),
    (
        Map::Route31,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (20, PokemonSpecies::Poliwag),
                (15, PokemonSpecies::Poliwag),
                (20, PokemonSpecies::Poliwhirl),
            ],
        },
    ),
    (
        Map::Route32,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (15, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Quagsire),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route34,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (20, PokemonSpecies::Tentacool),
                (15, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route35,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (20, PokemonSpecies::Psyduck),
                (15, PokemonSpecies::Psyduck),
                (20, PokemonSpecies::Golduck),
            ],
        },
    ),
    (
        Map::Route40,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (20, PokemonSpecies::Tentacool),
                (15, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route41,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (20, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Tentacruel),
                (20, PokemonSpecies::Mantine),
            ],
        },
    ),
    (
        Map::Route42,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (20, PokemonSpecies::Goldeen),
                (15, PokemonSpecies::Goldeen),
                (20, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::Route43,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (20, PokemonSpecies::Magikarp),
                (15, PokemonSpecies::Magikarp),
                (10, PokemonSpecies::Magikarp),
            ],
        },
    ),
    (
        Map::Route44,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (25, PokemonSpecies::Poliwag),
                (20, PokemonSpecies::Poliwag),
                (25, PokemonSpecies::Poliwhirl),
            ],
        },
    ),
    (
        Map::Route45,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (20, PokemonSpecies::Magikarp),
                (15, PokemonSpecies::Magikarp),
                (5, PokemonSpecies::Magikarp),
            ],
        },
    ),
    (
        Map::NewBarkTown,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (20, PokemonSpecies::Tentacool),
                (15, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::CherrygroveCity,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (20, PokemonSpecies::Tentacool),
                (15, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::VioletCity,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (20, PokemonSpecies::Poliwag),
                (15, PokemonSpecies::Poliwag),
                (20, PokemonSpecies::Poliwhirl),
            ],
        },
    ),
    (
        Map::CianwoodCity,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (20, PokemonSpecies::Tentacool),
                (15, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::OlivineCity,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (20, PokemonSpecies::Tentacool),
                (15, PokemonSpecies::Tentacool),
                (20, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::EcruteakCity,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (20, PokemonSpecies::Poliwag),
                (15, PokemonSpecies::Poliwag),
                (20, PokemonSpecies::Poliwhirl),
            ],
        },
    ),
    (
        Map::LakeOfRage,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (15, PokemonSpecies::Magikarp),
                (10, PokemonSpecies::Magikarp),
                (15, PokemonSpecies::Gyarados),
            ],
        },
    ),
    (
        Map::BlackthornCity,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (15, PokemonSpecies::Magikarp),
                (10, PokemonSpecies::Magikarp),
                (5, PokemonSpecies::Magikarp),
            ],
        },
    ),
    (
        Map::SilverCaveOutside,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (35, PokemonSpecies::Poliwhirl),
                (40, PokemonSpecies::Poliwhirl),
                (35, PokemonSpecies::Poliwag),
            ],
        },
    ),
];
