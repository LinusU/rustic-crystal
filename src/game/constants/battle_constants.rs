use crate::game::macros::r#enum::define_u8_enum;

pub const NUM_MOVES: u8 = 4;

pub const SUBSTATUS_TRANSFORMED: u8 = 3;

define_u8_enum! {
    pub enum BattleMode {
        Wild = 1,
        Trainer = 2,
    }
}

define_u8_enum! {
    pub enum BattleType {
        Debug = 2,
        Tutorial = 3,
        Contest = 6,
        Celebi = 11,
    }
}
