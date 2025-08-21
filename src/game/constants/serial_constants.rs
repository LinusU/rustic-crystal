use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum LinkMode {
        Null = 0,
        TimeCapsule = 1,
        TradeCenter = 2,
        Colosseum = 3,
        Mobile = 4,
    }
}

define_u8_enum! {
    pub enum SerialConnectionStatus {
        UsingExternalClock = 0x01,
        UsingInternalClock = 0x02,
        NotEstablished = 0xff,
    }
}
