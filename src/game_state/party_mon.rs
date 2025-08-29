use crate::{
    game::constants::{item_constants::Item, pokemon_constants::PokemonSpecies},
    game_state::{
        box_mon::{BoxMon, BoxMonMut},
        moveset::Moveset,
    },
};

pub const PARTYMON_STRUCT_LENGTH: usize = 48;

pub struct PartyMon<'a> {
    data: &'a [u8],
}

impl<'a> PartyMon<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data: &data[..PARTYMON_STRUCT_LENGTH],
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn species(&self) -> PokemonSpecies {
        BoxMon::new(self.data).species()
    }

    pub fn item(&self) -> Option<Item> {
        BoxMon::new(self.data).item()
    }

    pub fn moves(&self) -> Moveset {
        BoxMon::new(self.data).moves()
    }

    pub fn happiness(&self) -> u8 {
        BoxMon::new(self.data).happiness()
    }

    pub fn level(&self) -> u8 {
        BoxMon::new(self.data).level()
    }

    pub fn hp(&self) -> u16 {
        BoxMon::new(self.data).hp()
    }

    pub fn max_hp(&self) -> u16 {
        BoxMon::new(self.data).max_hp()
    }

    pub fn attack(&self) -> u16 {
        BoxMon::new(self.data).attack()
    }

    pub fn defense(&self) -> u16 {
        BoxMon::new(self.data).defense()
    }
}

pub struct PartyMonMut<'a> {
    data: &'a mut [u8],
}

impl<'a> PartyMonMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            data: &mut data[..PARTYMON_STRUCT_LENGTH],
        }
    }

    pub fn copy_from_slice(&mut self, src: &[u8]) {
        self.data.copy_from_slice(src);
    }

    pub fn set_species(&mut self, species: PokemonSpecies) {
        BoxMonMut::new(self.data).set_species(species);
    }

    pub fn set_item(&mut self, item: Option<Item>) {
        BoxMonMut::new(self.data).set_item(item);
    }

    pub fn set_happiness(&mut self, happiness: u8) {
        BoxMonMut::new(self.data).set_happiness(happiness)
    }

    pub fn set_hp(&mut self, hp: u16) {
        self.data[34..=35].copy_from_slice(&hp.to_be_bytes());
    }
}
