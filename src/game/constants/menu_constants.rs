use bitflags::bitflags;

pub const NAME_MON: u8 = 0;
pub const NAME_BOX: u8 = 4;

bitflags! {
    pub struct Menu2DFlags1: u8 {
        const WRAP_UP_DOWN = 1 << 5; // Wrap around when scrolling up/down
    }
}
