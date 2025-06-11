use bitflags::bitflags;

bitflags! {
    pub struct JoypadButtons: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const X = 1 << 2;
        const Y = 1 << 3;
        const L = 1 << 4;
        const R = 1 << 5;
        const START = 1 << 6;
        const SELECT = 1 << 7;
    }
}
