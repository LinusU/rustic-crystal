use crate::{
    game::constants::{battle_constants::NUM_MOVES, move_constants::Move},
    game_state::box_mon::{BoxMon, BoxMonMut},
};

pub const PARTYMON_STRUCT_LENGTH: usize = 48;

pub struct PartyMon<'a> {
    data: &'a [u8],
}

impl<'a> PartyMon<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn moves(&self) -> [Move; NUM_MOVES as usize] {
        BoxMon::new(self.data).moves()
    }
}

pub struct PartyMonMut<'a> {
    data: &'a mut [u8],
}

impl<'a> PartyMonMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    pub fn set_happiness(&mut self, happiness: u8) {
        BoxMonMut::new(self.data).set_happiness(happiness)
    }
}
