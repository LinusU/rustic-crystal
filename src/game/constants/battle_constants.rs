pub const NUM_MOVES: u8 = 4;

pub const SUBSTATUS_TRANSFORMED: u8 = 3;

pub enum BattleMode {
    Wild,
    Trainer,
}

impl BattleMode {
    pub fn from_u8(value: u8) -> Option<BattleMode> {
        match value {
            0 => None,
            1 => Some(BattleMode::Wild),
            2 => Some(BattleMode::Trainer),
            n => panic!("Invalid BattleMode value: {}", n),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            BattleMode::Wild => 1,
            BattleMode::Trainer => 2,
        }
    }
}

pub enum BattleType {
    Debug,
    Tutorial,
    Contest,
    Celebi,
}

impl BattleType {
    pub fn to_u8(&self) -> u8 {
        match self {
            BattleType::Debug => 2,
            BattleType::Tutorial => 3,
            BattleType::Contest => 6,
            BattleType::Celebi => 11,
        }
    }
}
