use bitflags::bitflags;

bitflags! {
    pub struct UnlockedUnowns: u8 {
        const A_TO_K = 1 << 0;
        const L_TO_R = 1 << 1;
        const S_TO_W = 1 << 2;
        const X_TO_Z = 1 << 3;
    }
}
