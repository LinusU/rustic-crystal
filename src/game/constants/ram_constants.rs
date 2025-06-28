pub const NO_TEXT_SCROLL: u8 = 4;

pub enum MonType {
    Box,
}

impl MonType {
    pub fn to_u8(self) -> u8 {
        match self {
            Self::Box => 2,
        }
    }
}
