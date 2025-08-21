use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum Item {
        MasterBall = 0x01,
        UltraBall = 0x02,
        GreatBall = 0x04,
        PokeBall = 0x05,
        MoonStone = 0x08,
        FireStone = 0x16,
        Thunderstone = 0x17,
        WaterStone = 0x18,
        LeafStone = 0x22,
        KingsRock = 0x52,
        Everstone = 0x70,
        MetalCoat = 0x8f,
        DragonScale = 0x97,
        HeavyBall = 0x9d,
        LevelBall = 0x9f,
        LureBall = 0xa0,
        FastBall = 0xa1,
        FriendBall = 0xa4,
        MoonBall = 0xa5,
        LoveBall = 0xa6,
        SunStone = 0xa9,
        UpGrade = 0xac,
        ParkBall = 0xb1,
        BrickPiece = 0xb4,
    }
}

pub const SAFARI_BALL: Item = Item::MoonStone; // leftovers from red
