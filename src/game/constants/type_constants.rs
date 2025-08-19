use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum Type {
        Normal = 0,
        Fighting = 1,
        Flying = 2,
        Poison = 3,
        Ground = 4,
        Rock = 5,
        Bird = 6,
        Bug = 7,
        Ghost = 8,
        Steel = 9,
        Curse = 19,
        Fire = 20,
        Water = 21,
        Grass = 22,
        Electric = 23,
        Psychic = 24,
        Ice = 25,
        Dragon = 26,
        Dark = 27,
    }
}
