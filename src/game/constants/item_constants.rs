use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum Item {
        MasterBall = 0x01,
        UltraBall = 0x02,
        GreatBall = 0x04,
        PokeBall = 0x05,
        MoonStone = 0x08,
        HeavyBall = 0x9d,
        LevelBall = 0x9f,
        LureBall = 0xa0,
        FastBall = 0xa1,
        FriendBall = 0xa4,
        MoonBall = 0xa5,
        LoveBall = 0xa6,
        ParkBall = 0xb1,
    }
}

pub const SAFARI_BALL: Item = Item::MoonStone; // leftovers from red
