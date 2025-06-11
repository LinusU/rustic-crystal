use bitflags::bitflags;

bitflags! {
    pub struct Menu2DFlags1: u8 {
        const WRAP_UP_DOWN = 1 << 5; // Wrap around when scrolling up/down
    }
}
