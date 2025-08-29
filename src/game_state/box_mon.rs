use crate::{
    game::constants::{item_constants::Item, pokemon_constants::PokemonSpecies},
    game_state::moveset::Moveset,
};

pub const BOXMON_STRUCT_LENGTH: usize = 32;

pub struct BoxMon<'a> {
    data: &'a [u8],
}

impl<'a> BoxMon<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data: &data[..BOXMON_STRUCT_LENGTH],
        }
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

    pub fn happiness(&self) -> u8 {
        self.data[27]
    }

    pub fn level(&self) -> u8 {
        self.data[31]
    }
}

pub struct BoxMonMut<'a> {
    data: &'a mut [u8],
}

impl<'a> BoxMonMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    pub fn set_species(&mut self, species: PokemonSpecies) {
        self.data[0] = species.into();
    }

    pub fn set_item(&mut self, item: Option<Item>) {
        self.data[1] = match item {
            None => 0,
            Some(i) => i.into(),
        };
    }

    pub fn set_happiness(&mut self, happiness: u8) {
        self.data[27] = happiness;
    }
}
