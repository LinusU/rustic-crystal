use crate::{
    game::constants::{pokemon_constants::PokemonSpecies, pokemon_data_constants::GrowthRate},
    rom::ROM,
};

pub const BASE_STATS: usize = (0x14 * 0x4000) | (0x5424 & 0x3fff);
pub const BASE_DATA_SIZE: usize = 32;

impl PokemonSpecies {
    pub fn growth_rate(self) -> GrowthRate {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (u8::from(self) as usize - 1)) + 22].into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_rate() {
        assert_eq!(
            PokemonSpecies::Magnemite.growth_rate(),
            GrowthRate::MediumFast
        );

        assert_eq!(PokemonSpecies::Dragonair.growth_rate(), GrowthRate::Slow);
        assert_eq!(PokemonSpecies::Entei.growth_rate(), GrowthRate::Slow);
    }
}
