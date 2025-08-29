#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DeterminantValues(u8, u8);

impl DeterminantValues {
    pub fn hp(&self) -> u8 {
        ((self.attack() & 1) << 3)
            | ((self.defense() & 1) << 2)
            | ((self.speed() & 1) << 1)
            | (self.special() & 1)
    }

    pub fn attack(&self) -> u8 {
        (self.0 & 0b1111_0000) >> 4
    }

    pub fn defense(&self) -> u8 {
        self.0 & 0b0000_1111
    }

    pub fn speed(&self) -> u8 {
        (self.1 & 0b1111_0000) >> 4
    }

    pub fn special(&self) -> u8 {
        self.1 & 0b0000_1111
    }
}

impl From<[u8; 2]> for DeterminantValues {
    fn from(bytes: [u8; 2]) -> Self {
        DeterminantValues(bytes[0], bytes[1])
    }
}

impl std::fmt::Debug for DeterminantValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeterminantValues")
            .field("hp", &self.hp())
            .field("attack", &self.attack())
            .field("defense", &self.defense())
            .field("speed", &self.speed())
            .field("special", &self.special())
            .finish()
    }
}
