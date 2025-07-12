use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum SerialConnectionStatus {
        UsingExternalClock = 0x01,
        UsingInternalClock = 0x02,
        NotEstablished = 0xff,
    }
}
