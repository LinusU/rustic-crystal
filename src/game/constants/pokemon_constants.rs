pub enum PokemonSpecies {
    Ditto,
}

impl PokemonSpecies {
    pub fn to_u8(self) -> u8 {
        match self {
            PokemonSpecies::Ditto => 0x84,
        }
    }
}
