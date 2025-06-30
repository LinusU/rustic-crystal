use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum MonType {
        Party = 0,
        Box = 2,
    }
}
