use bitflags::bitflags;

pub const BOX_NAME_LENGTH: usize = 9;
pub const NAME_LENGTH: usize = 11;
pub const MON_NAME_LENGTH: usize = 11;
pub const MOVE_NAME_LENGTH: usize = 13;

bitflags! {
    pub struct PrintNum: u8 {
        const MONEY = 1 << 5;
        const LEFT_ALIGN = 1 << 6;
        const LEADING_ZEROS = 1 << 7;
    }
}
