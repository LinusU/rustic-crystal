use crate::{
    game::constants::{
        pokemon_constants::PokemonSpecies, pokemon_data_constants::GrowthRate, type_constants::Type,
    },
    rom::ROM,
};

const BASE_STATS: usize = (0x14 * 0x4000) | (0x5424 & 0x3fff);
const BASE_DATA_SIZE: usize = 32;

impl PokemonSpecies {
    pub fn types(self) -> (Type, Type) {
        let offset = BASE_STATS + (BASE_DATA_SIZE * (u8::from(self) as usize - 1)) + 7;

        (ROM[offset].into(), ROM[offset + 1].into())
    }

    pub fn base_hp(self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (u8::from(self) as usize - 1)) + 1]
    }

    pub fn base_attack(self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (u8::from(self) as usize - 1)) + 2]
    }

    pub fn base_defense(self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (u8::from(self) as usize - 1)) + 3]
    }

    pub fn base_speed(self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (u8::from(self) as usize - 1)) + 4]
    }

    pub fn base_special_attack(self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (u8::from(self) as usize - 1)) + 5]
    }

    pub fn base_special_defense(self) -> u8 {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (u8::from(self) as usize - 1)) + 6]
    }

    pub fn growth_rate(self) -> GrowthRate {
        ROM[BASE_STATS + (BASE_DATA_SIZE * (u8::from(self) as usize - 1)) + 22].into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types() {
        assert_eq!(
            PokemonSpecies::Magnemite.types(),
            (Type::Electric, Type::Steel)
        );

        assert_eq!(
            PokemonSpecies::Dragonair.types(),
            (Type::Dragon, Type::Dragon)
        );

        assert_eq!(PokemonSpecies::Entei.types(), (Type::Fire, Type::Fire));
    }

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
