use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum PokemonSpecies {
        Magnemite = 0x51,
        Grimer = 0x58,
        Tangela = 0x72,
        MrMime = 0x7a,
        Ditto = 0x84,
        Eevee = 0x85,
        Porygon = 0x89,
        Dratini = 0x93,
        Dragonair = 0x94,
        Togetic = 0xb0,
        Umbreon = 0xc5,
        Unown = 0xc9,
        Snubbull = 0xd1,
        Heracross = 0xd6,
    }
}
