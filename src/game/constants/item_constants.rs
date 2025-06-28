use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum Item {
        MasterBall = 0x01,
        PokeBall = 0x05,
        LevelBall = 0x9f,
        FriendBall = 0xa4,
        ParkBall = 0xb1,
    }
}
