use crate::game::constants::pokemon_constants::PokemonSpecies;

pub const SOMETIMES_FLEE_MONS: [PokemonSpecies; 13] = [
    PokemonSpecies::Magnemite,
    PokemonSpecies::Grimer,
    PokemonSpecies::Tangela,
    PokemonSpecies::MrMime,
    PokemonSpecies::Eevee,
    PokemonSpecies::Porygon,
    PokemonSpecies::Dratini,
    PokemonSpecies::Dragonair,
    PokemonSpecies::Togetic,
    PokemonSpecies::Umbreon,
    PokemonSpecies::Unown,
    PokemonSpecies::Snubbull,
    PokemonSpecies::Heracross,
];

pub const OFTEN_FLEE_MONS: [PokemonSpecies; 8] = [
    PokemonSpecies::Cubone,
    PokemonSpecies::Articuno,
    PokemonSpecies::Zapdos,
    PokemonSpecies::Moltres,
    PokemonSpecies::Quagsire,
    PokemonSpecies::Delibird,
    PokemonSpecies::Phanpy,
    PokemonSpecies::Teddiursa,
];

pub const ALWAYS_FLEE_MONS: [PokemonSpecies; 2] = [PokemonSpecies::Raikou, PokemonSpecies::Entei];
