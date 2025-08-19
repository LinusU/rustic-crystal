pub const PARTYMON_STRUCT_LENGTH: usize = 48;

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
