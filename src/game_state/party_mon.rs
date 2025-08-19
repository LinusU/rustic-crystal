use crate::game::constants::{battle_constants::NUM_MOVES, move_constants::Move};

pub const PARTYMON_STRUCT_LENGTH: usize = 48;

pub struct PartyMon<'a> {
    data: &'a [u8],
}

impl<'a> PartyMon<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
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

pub struct PartyMonMut<'a> {
    data: &'a mut [u8],
}

impl<'a> PartyMonMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    pub fn set_happiness(&mut self, happiness: u8) {
        self.data[27] = happiness;
    }
}
