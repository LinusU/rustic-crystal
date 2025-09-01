use crate::{
    game::constants::{
        item_constants::Item, pokemon_constants::PokemonSpecies, type_constants::Type,
    },
    game_state::moveset::Moveset,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleMonStatus(u8);

impl BattleMonStatus {
    pub fn is_sleeping(&self) -> bool {
        self.0 & 0b0000_0111 != 0 // 0-7 turns
    }

    pub fn is_poisoned(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub fn is_burned(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    pub fn is_frozen(&self) -> bool {
        self.0 & 0b0010_0000 != 0
    }

    pub fn is_paralyzed(&self) -> bool {
        self.0 & 0b0100_0000 != 0
    }
}

impl From<u8> for BattleMonStatus {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<BattleMonStatus> for u8 {
    fn from(val: BattleMonStatus) -> Self {
        val.0
    }
}

pub struct BattleMon<'a> {
    data: &'a [u8],
}

impl<'a> BattleMon<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn species(&self) -> PokemonSpecies {
        self.data[0].into()
    }

    pub fn item(&self) -> Option<Item> {
        match self.data[1] {
            0 => None,
            n => Some(n.into()),
        }
    }

    pub fn moves(&self) -> Moveset {
        [self.data[2], self.data[3], self.data[4], self.data[5]].into()
    }

    pub fn dvs(&self) -> u16 {
        u16::from_be_bytes([self.data[6], self.data[7]])
    }

    pub fn pp(&self) -> [u8; 4] {
        [self.data[8], self.data[9], self.data[10], self.data[11]]
    }

    pub fn level(&self) -> u8 {
        self.data[13]
    }

    pub fn status(&self) -> BattleMonStatus {
        BattleMonStatus(self.data[14])
    }

    pub fn hp(&self) -> u16 {
        u16::from_be_bytes([self.data[16], self.data[17]])
    }

    pub fn max_hp(&self) -> u16 {
        u16::from_be_bytes([self.data[18], self.data[19]])
    }

    pub fn types(&self) -> (Type, Type) {
        (
            self.data[30].into(), // Primary type
            self.data[31].into(), // Secondary type
        )
    }
}

pub struct BattleMonMut<'a> {
    data: &'a mut [u8],
}

impl<'a> BattleMonMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    pub fn set_species(&mut self, species: PokemonSpecies) {
        self.data[0] = species.into();
    }

    pub fn set_item(&mut self, item: Option<Item>) -> &mut Self {
        self.data[1] = item.map_or(0, Into::into);
        self
    }

    pub fn set_dvs(&mut self, dvs: u16) {
        self.data[6..8].copy_from_slice(&dvs.to_be_bytes());
    }

    pub fn set_level(&mut self, level: u8) {
        self.data[13] = level;
    }

    pub fn set_status(&mut self, status: BattleMonStatus) {
        self.data[14] = status.into();
    }

    pub fn set_hp(&mut self, hp: u16) -> &mut Self {
        self.data[16..18].copy_from_slice(&hp.to_be_bytes());
        self
    }

    pub fn set_max_hp(&mut self, max_hp: u16) {
        self.data[18..20].copy_from_slice(&max_hp.to_be_bytes());
    }

    pub fn set_types(&mut self, types: (Type, Type)) {
        self.data[30] = types.0.into();
        self.data[31] = types.1.into();
    }
}
