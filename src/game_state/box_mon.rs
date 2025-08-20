use crate::game::constants::{
    battle_constants::NUM_MOVES, move_constants::Move, pokemon_constants::PokemonSpecies,
};

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

    pub fn moves(&self) -> [Move; NUM_MOVES as usize] {
        [
            self.data[2].into(),
            self.data[3].into(),
            self.data[4].into(),
            self.data[5].into(),
        ]
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
