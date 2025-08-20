use crate::{game::constants::pokemon_constants::PokemonSpecies, game_state::moveset::Moveset};

pub struct BoxMon<'a> {
    data: &'a [u8],
}

impl<'a> BoxMon<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn species(&self) -> PokemonSpecies {
        self.data[0].into()
    }

    pub fn moves(&self) -> Moveset {
        [self.data[2], self.data[3], self.data[4], self.data[5]].into()
    }
}

pub struct BoxMonMut<'a> {
    data: &'a mut [u8],
}

impl<'a> BoxMonMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    pub fn set_happiness(&mut self, happiness: u8) {
        self.data[27] = happiness;
    }
}
