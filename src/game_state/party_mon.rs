use crate::{
    game::constants::{item_constants::Item, pokemon_constants::PokemonSpecies},
    game_state::{
        box_mon::{BoxMonMut, BoxMonRef},
        mon_list::MonListItem,
        moveset::Moveset,
    },
};

pub struct PartyMonRef<'a> {
    data: &'a [u8],
}

impl<'a> PartyMonRef<'a> {
    pub const LEN: usize = 48;

    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data: &data[..Self::LEN],
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn species(&self) -> PokemonSpecies {
        BoxMonRef::new(self.data).species()
    }

    pub fn item(&self) -> Option<Item> {
        BoxMonRef::new(self.data).item()
    }

    pub fn moves(&self) -> Moveset {
        BoxMonRef::new(self.data).moves()
    }

    pub fn happiness(&self) -> u8 {
        BoxMonRef::new(self.data).happiness()
    }

    pub fn level(&self) -> u8 {
        BoxMonRef::new(self.data).level()
    }

    pub fn hp(&self) -> u16 {
        u16::from_be_bytes([self.data[34], self.data[35]])
    }

    pub fn max_hp(&self) -> u16 {
        u16::from_be_bytes([self.data[36], self.data[37]])
    }

    pub fn attack(&self) -> u16 {
        u16::from_be_bytes([self.data[38], self.data[39]])
    }

    pub fn defense(&self) -> u16 {
        u16::from_be_bytes([self.data[40], self.data[41]])
    }
}

impl<'a> MonListItem<'a> for PartyMonRef<'a> {
    const LEN: usize = PartyMonRef::LEN;

    fn new(data: &'a [u8]) -> Self {
        PartyMonRef::new(data)
    }
}

pub struct PartyMonMut<'a> {
    data: &'a mut [u8],
}

impl<'a> PartyMonMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            data: &mut data[..PartyMonRef::LEN],
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
