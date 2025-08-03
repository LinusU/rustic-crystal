use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum MonType {
        Party = 0,
        Box = 2,
    }
}

define_u8_enum! {
    pub enum TimeOfDay {
        Morn = 0,
        Day = 1,
        Nite = 2,
    }
}
