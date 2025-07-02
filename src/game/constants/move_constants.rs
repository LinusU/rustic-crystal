use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum Move {
        Pound = 0x01,
    }
}

pub const ANIM_THROW_POKE_BALL: u16 = 0x0100;
