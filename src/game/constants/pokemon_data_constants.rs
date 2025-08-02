use crate::game::macros::r#enum::define_u8_enum;

/// Maximum number of party pokemon
pub const PARTY_LENGTH: u8 = 6;

pub const MONS_PER_BOX: u8 = 20;
pub const NUM_BOXES: u8 = 14;

pub const NUM_HOF_TEAMS: u8 = 30;

pub const NUM_GRASSMON: usize = 7; // data/wild/*_grass.asm table size
pub const NUM_WATERMON: usize = 3; // data/wild/*_water.asm table size

pub const GRASS_WILDDATA_LENGTH: usize = 2 + 3 + NUM_GRASSMON * 2 * 3;
pub const WATER_WILDDATA_LENGTH: usize = 2 + 1 + NUM_WATERMON * 2;
pub const FISHGROUP_DATA_LENGTH: usize = 1 + 2 * 3;

pub const PARTYMON_STRUCT_LENGTH: u8 = 48;

pub const BASE_HAPPINESS: u8 = 70;
pub const FRIEND_BALL_HAPPINESS: u8 = 200;

define_u8_enum! {
    pub enum GrowthRate {
        MediumFast = 0,
        SlightlyFast = 1,
        SlightlySlow = 2,
        MediumSlow = 3,
        Fast = 4,
        Slow = 5,
    }
}

impl GrowthRate {
    pub fn exp_at_level(self, level: u8) -> u32 {
        let level = level as u32;

        match self {
            Self::MediumFast => level.pow(3),
            Self::SlightlyFast => (level.pow(3) * 3 / 4) + (level.pow(2) * 10) - 30,
            Self::SlightlySlow => (level.pow(3) * 3 / 4) + (level.pow(2) * 20) - 70,
            Self::MediumSlow => (level.pow(3) * 6 / 5) + (level * 100) - (level.pow(2) * 15) - 140,
            Self::Fast => level.pow(3) * 4 / 5,
            Self::Slow => level.pow(3) * 5 / 4,

            Self::Unknown(n) => {
                log::error!("Trying to calculate exp for unknown growth rate: {n}");
                0
            }
        }
    }
}

define_u8_enum! {
    pub enum EvolutionType {
        Level = 1,
        Item = 2,
        Trade = 3,
        Happiness = 4,
        Stat = 5,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exp_at_level() {
        assert_eq!(GrowthRate::Fast.exp_at_level(2), 6);
        assert_eq!(GrowthRate::MediumFast.exp_at_level(2), 8);
        assert_eq!(GrowthRate::MediumSlow.exp_at_level(2), 9);
        assert_eq!(GrowthRate::Slow.exp_at_level(2), 10);

        assert_eq!(GrowthRate::Fast.exp_at_level(13), 1_757);
        assert_eq!(GrowthRate::MediumFast.exp_at_level(13), 2_197);
        assert_eq!(GrowthRate::MediumSlow.exp_at_level(13), 1_261);
        assert_eq!(GrowthRate::Slow.exp_at_level(13), 2_746);

        assert_eq!(GrowthRate::Fast.exp_at_level(97), 730_138);
        assert_eq!(GrowthRate::MediumFast.exp_at_level(97), 912_673);
        assert_eq!(GrowthRate::MediumSlow.exp_at_level(97), 963_632);
        assert_eq!(GrowthRate::Slow.exp_at_level(97), 1_140_841);

        assert_eq!(GrowthRate::SlightlyFast.exp_at_level(100), 849_970);
        assert_eq!(GrowthRate::SlightlySlow.exp_at_level(100), 949_930);
    }
}
