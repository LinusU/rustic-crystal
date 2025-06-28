pub enum Item {
    MasterBall,
    PokeBall,
    LevelBall,
    FriendBall,
    ParkBall,
}

impl Item {
    pub fn to_u8(self) -> u8 {
        match self {
            Item::MasterBall => 0x01,
            Item::PokeBall => 0x05,
            Item::LevelBall => 0x9f,
            Item::FriendBall => 0xa4,
            Item::ParkBall => 0xb1,
        }
    }
}
