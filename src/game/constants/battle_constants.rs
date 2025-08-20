use bitflags::bitflags;

use crate::game::macros::r#enum::define_u8_enum;

pub const NUM_EXP_STATS: u8 = 5;
pub const NUM_MOVES: usize = 4;

pub const SUBSTATUS_TRANSFORMED: u8 = 3;

define_u8_enum! {
    pub enum TypeEffectiveness {
        SuperEffective = 20,
        MoreEffective = 15,
        Effective = 10,
        NotVeryEffective = 5,
        NoEffect = 0,
    }
}

bitflags! {
    pub struct BattleResult: u8 {
        const LOSE          = 1 << 0;
        const DRAW          = 1 << 1;
        const CAUGHT_CELEBI = 1 << 6;
        const BOX_FULL      = 1 << 7;
    }
}

define_u8_enum! {
    pub enum BattleMode {
        Wild = 1,
        Trainer = 2,
    }
}

define_u8_enum! {
    pub enum BattleType {
        Normal = 0,
        CanLose = 1,
        Debug = 2,
        Tutorial = 3,
        Fish = 4,
        Roaming = 5,
        Contest = 6,
        ForceShiny = 7,
        Tree = 8,
        Trap = 9,
        ForceItem = 10,
        Celebi = 11,
        Suicune = 12,
    }
}
