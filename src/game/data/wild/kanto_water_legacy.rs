use crate::game::{
    constants::{map_constants::Map, pokemon_constants::PokemonSpecies},
    macros::{asserts::WaterWildmons, data::percent},
};

pub const KANTO_WATER_WILD_MONS: &[(Map, WaterWildmons)] = &[
    (
        Map::TohjoFalls,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (20, PokemonSpecies::Goldeen),
                (20, PokemonSpecies::Slowpoke),
                (20, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::VermilionPort,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (35, PokemonSpecies::Tentacool),
                (30, PokemonSpecies::Tentacool),
                (35, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route4,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (10, PokemonSpecies::Goldeen),
                (5, PokemonSpecies::Goldeen),
                (10, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::Route6,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (10, PokemonSpecies::Psyduck),
                (5, PokemonSpecies::Psyduck),
                (10, PokemonSpecies::Golduck),
            ],
        },
    ),
    (
        Map::Route9,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (15, PokemonSpecies::Goldeen),
                (10, PokemonSpecies::Goldeen),
                (15, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::Route10North,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (15, PokemonSpecies::Goldeen),
                (10, PokemonSpecies::Goldeen),
                (15, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::Route12,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (25, PokemonSpecies::Tentacool),
                (25, PokemonSpecies::Quagsire),
                (25, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route13,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (25, PokemonSpecies::Tentacool),
                (25, PokemonSpecies::Quagsire),
                (25, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route19,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (35, PokemonSpecies::Tentacool),
                (30, PokemonSpecies::Tentacool),
                (35, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route20,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (35, PokemonSpecies::Tentacool),
                (30, PokemonSpecies::Tentacool),
                (35, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route21,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (35, PokemonSpecies::Tentacool),
                (30, PokemonSpecies::Tentacool),
                (35, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route22,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (10, PokemonSpecies::Poliwag),
                (5, PokemonSpecies::Poliwag),
                (10, PokemonSpecies::Poliwhirl),
            ],
        },
    ),
    (
        Map::Route24,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (10, PokemonSpecies::Goldeen),
                (5, PokemonSpecies::Goldeen),
                (10, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::Route25,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (10, PokemonSpecies::Goldeen),
                (5, PokemonSpecies::Goldeen),
                (10, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::Route26,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (30, PokemonSpecies::Tentacool),
                (25, PokemonSpecies::Tentacool),
                (30, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::Route27,
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
        Map::Route28,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (40, PokemonSpecies::Poliwag),
                (35, PokemonSpecies::Poliwag),
                (40, PokemonSpecies::Poliwhirl),
            ],
        },
    ),
    (
        Map::PalletTown,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (35, PokemonSpecies::Tentacool),
                (30, PokemonSpecies::Tentacool),
                (35, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::ViridianCity,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (10, PokemonSpecies::Poliwag),
                (5, PokemonSpecies::Poliwag),
                (10, PokemonSpecies::Poliwhirl),
            ],
        },
    ),
    (
        Map::CeruleanCity,
        WaterWildmons {
            encounter_rate: percent(4),
            encounters: [
                (10, PokemonSpecies::Goldeen),
                (5, PokemonSpecies::Goldeen),
                (10, PokemonSpecies::Seaking),
            ],
        },
    ),
    (
        Map::VermilionCity,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (35, PokemonSpecies::Tentacool),
                (30, PokemonSpecies::Tentacool),
                (35, PokemonSpecies::Tentacruel),
            ],
        },
    ),
    (
        Map::CeladonCity,
        WaterWildmons {
            encounter_rate: percent(2),
            encounters: [
                (20, PokemonSpecies::Grimer),
                (15, PokemonSpecies::Grimer),
                (15, PokemonSpecies::Muk),
            ],
        },
    ),
    (
        Map::FuchsiaCity,
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
        Map::CinnabarIsland,
        WaterWildmons {
            encounter_rate: percent(6),
            encounters: [
                (35, PokemonSpecies::Tentacool),
                (30, PokemonSpecies::Tentacool),
                (35, PokemonSpecies::Tentacruel),
            ],
        },
    ),
];
