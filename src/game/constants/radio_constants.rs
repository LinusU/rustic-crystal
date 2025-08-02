use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum RadioChannelId {
        OaksPokemonTalk = 0x00,

        OaksPokemonTalk5 = 0x0e,
    }
}
