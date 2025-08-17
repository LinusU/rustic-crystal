use crate::game::{
    constants::{map_constants::Map, pokemon_constants::PokemonSpecies},
    macros::{asserts::GrassWildmons, data::percent},
};

pub const SWARM_GRASS_WILD_MONS: &[(Map, GrassWildmons)] = &[
    // Dunsparce swarm
    (
        Map::DarkCaveVioletEntrance,
        GrassWildmons {
            encounter_rates: [percent(4), percent(4), percent(4)],
            morn: [
                (3, PokemonSpecies::Geodude),
                (3, PokemonSpecies::Dunsparce),
                (2, PokemonSpecies::Zubat),
                (2, PokemonSpecies::Geodude),
                (2, PokemonSpecies::Dunsparce),
                (4, PokemonSpecies::Dunsparce),
                (4, PokemonSpecies::Dunsparce),
            ],
            day: [
                (3, PokemonSpecies::Geodude),
                (3, PokemonSpecies::Dunsparce),
                (2, PokemonSpecies::Zubat),
                (2, PokemonSpecies::Geodude),
                (2, PokemonSpecies::Dunsparce),
                (4, PokemonSpecies::Dunsparce),
                (4, PokemonSpecies::Dunsparce),
            ],
            nite: [
                (3, PokemonSpecies::Geodude),
                (3, PokemonSpecies::Dunsparce),
                (2, PokemonSpecies::Zubat),
                (2, PokemonSpecies::Geodude),
                (2, PokemonSpecies::Dunsparce),
                (4, PokemonSpecies::Dunsparce),
                (4, PokemonSpecies::Dunsparce),
            ],
        },
    ),
    // Yanma swarm
    (
        Map::Route35,
        GrassWildmons {
            encounter_rates: [percent(10), percent(10), percent(10)],
            morn: [
                (12, PokemonSpecies::NidoranM),
                (12, PokemonSpecies::NidoranF),
                (12, PokemonSpecies::Yanma),
                (14, PokemonSpecies::Yanma),
                (14, PokemonSpecies::Pidgey),
                (10, PokemonSpecies::Ditto),
                (10, PokemonSpecies::Ditto),
            ],
            day: [
                (12, PokemonSpecies::NidoranM),
                (12, PokemonSpecies::NidoranF),
                (12, PokemonSpecies::Yanma),
                (14, PokemonSpecies::Yanma),
                (14, PokemonSpecies::Pidgey),
                (10, PokemonSpecies::Ditto),
                (10, PokemonSpecies::Ditto),
            ],
            nite: [
                (12, PokemonSpecies::NidoranM),
                (12, PokemonSpecies::NidoranF),
                (12, PokemonSpecies::Yanma),
                (14, PokemonSpecies::Yanma),
                (14, PokemonSpecies::Hoothoot),
                (10, PokemonSpecies::Ditto),
                (10, PokemonSpecies::Ditto),
            ],
        },
    ),
];
