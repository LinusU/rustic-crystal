use bitflags::bitflags;

use crate::game::macros::r#enum::define_u8_enum;

define_u8_enum! {
    pub enum MonType {
        Party = 0,
        OtherParty = 1,
        Box = 2,
        Temp = 3,
    }
}

define_u8_enum! {
    pub enum PokemonWithdrawDepositParameter {
        PCWithdraw = 0,
        PCDeposit = 1,
    }
}

define_u8_enum! {
    pub enum TimeOfDay {
        Morn = 0,
        Day = 1,
        Nite = 2,
    }
}

bitflags! {
    pub struct StatusFlags: u8 {
        const POKEDEX = 1 << 0;
        const UNOWN_DEX = 1 << 1;
        const FLASH = 1 << 2;
        const CAUGHT_POKERUS = 1 << 3;
        const ROCKET_SIGNAL = 1 << 4;
        const NO_WILD_ENCOUNTERS = 1 << 5;
        const HALL_OF_FAME = 1 << 6;
        const MAIN_MENU_MOBILE_CHOICES = 1 << 7;
    }
}

bitflags! {
    pub struct SwarmFlags: u8 {
        const BUENAS_PASSWORD = 1 << 0;
        const GOLDENROD_DEPT_STORE_SALE = 1 << 1;
        const DUNSPARCE_SWARM = 1 << 2;
        const YANMA_SWARM = 1 << 3;
        const MOBILE_4 = 1 << 4;
    }
}
