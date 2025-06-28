use crate::game::macros::r#enum::define_u8_enum;

pub const NO_TEXT_SCROLL: u8 = 4;

define_u8_enum! {
    pub enum MonType {
        Box = 2,
    }
}
