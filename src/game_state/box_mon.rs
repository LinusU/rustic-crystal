use std::fmt::Debug;

use crate::{
    game::constants::{
        item_constants::Item, pokemon_constants::PokemonSpecies,
        pokemon_data_constants::BASE_HAPPINESS,
    },
    game_state::{battle_mon::BattleMon, moveset::Moveset, party_mon::PartyMon},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxMonOwned {
    data: [u8; BoxMonOwned::LEN],
}

impl BoxMonOwned {
    pub const LEN: usize = 32;

    pub fn as_ref(&self) -> BoxMonRef<'_> {
        BoxMonRef::new(&self.data)
    }

    pub fn from_battle_mon(battle_mon: BattleMon, ot_id: u16) -> Self {
        let mut result = Self {
            data: [0; Self::LEN],
        };

        result.set_species(battle_mon.species());
        result.set_item(battle_mon.item());
        result.set_moves(battle_mon.moves());
        result.set_ot_id(ot_id);
        result.set_exp(
            battle_mon
                .species()
                .growth_rate()
                .exp_at_level(battle_mon.level()),
        );
        result.set_dvs(battle_mon.dvs());
        result.set_pp(battle_mon.pp());
        result.set_happiness(BASE_HAPPINESS);
        result.set_level(battle_mon.level());

        result
    }

    pub fn from_party_mon(party_mon: PartyMon) -> Self {
        Self {
            // PartyMon is a superset of BoxMon, so this is safe
            data: party_mon.to_vec()[0..BoxMonOwned::LEN].try_into().unwrap(),
        }
    }

    pub fn set_species(&mut self, species: PokemonSpecies) {
        BoxMonMut::new(&mut self.data).set_species(species);
    }

    pub fn set_item(&mut self, item: Option<Item>) {
        BoxMonMut::new(&mut self.data).set_item(item);
    }

    pub fn set_moves(&mut self, moves: Moveset) {
        BoxMonMut::new(&mut self.data).set_moves(moves);
    }

    pub fn set_ot_id(&mut self, ot_id: u16) {
        BoxMonMut::new(&mut self.data).set_ot_id(ot_id);
    }

    pub fn set_exp(&mut self, exp: u32) {
        BoxMonMut::new(&mut self.data).set_exp(exp);
    }

    pub fn set_dvs(&mut self, dvs: u16) {
        BoxMonMut::new(&mut self.data).set_dvs(dvs);
    }

    pub fn set_pp(&mut self, pp: [u8; 4]) {
        BoxMonMut::new(&mut self.data).set_pp(pp);
    }

    pub fn set_happiness(&mut self, happiness: u8) {
        BoxMonMut::new(&mut self.data).set_happiness(happiness);
    }

    pub fn set_level(&mut self, level: u8) {
        BoxMonMut::new(&mut self.data).set_level(level);
    }
}

#[derive(PartialEq, Eq)]
pub struct BoxMonRef<'a> {
    data: &'a [u8],
}

impl<'a> BoxMonRef<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data: &data[..BoxMonOwned::LEN],
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

    pub fn ot_id(&self) -> u16 {
        u16::from_be_bytes([self.data[6], self.data[7]])
    }

    pub fn exp(&self) -> u32 {
        u32::from_be_bytes([0, self.data[8], self.data[9], self.data[10]])
    }

    pub fn hp_ev(&self) -> u16 {
        u16::from_be_bytes([self.data[11], self.data[12]])
    }

    pub fn attack_ev(&self) -> u16 {
        u16::from_be_bytes([self.data[13], self.data[14]])
    }

    pub fn defense_ev(&self) -> u16 {
        u16::from_be_bytes([self.data[15], self.data[16]])
    }

    pub fn speed_ev(&self) -> u16 {
        u16::from_be_bytes([self.data[17], self.data[18]])
    }

    pub fn special_ev(&self) -> u16 {
        u16::from_be_bytes([self.data[19], self.data[20]])
    }

    pub fn dvs(&self) -> u16 {
        u16::from_be_bytes([self.data[21], self.data[22]])
    }

    pub fn pp(&self) -> [u8; 4] {
        [self.data[23], self.data[24], self.data[25], self.data[26]]
    }

    pub fn happiness(&self) -> u8 {
        self.data[27]
    }

    pub fn pokerus_status(&self) -> u8 {
        self.data[28]
    }

    pub fn caught_data(&self) -> u16 {
        u16::from_be_bytes([self.data[29], self.data[30]])
    }

    pub fn level(&self) -> u8 {
        self.data[31]
    }
}

impl<'a> AsRef<[u8]> for BoxMonRef<'a> {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}

impl Debug for BoxMonRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxMonRef")
            .field("species", &self.species())
            .field("item", &self.item())
            .field("moves", &self.moves())
            .field("ot_id", &self.ot_id())
            .field("exp", &self.exp())
            .field("hp_ev", &self.hp_ev())
            .field("attack_ev", &self.attack_ev())
            .field("defense_ev", &self.defense_ev())
            .field("speed_ev", &self.speed_ev())
            .field("special_ev", &self.special_ev())
            .field("dvs", &self.dvs())
            .field("pp", &self.pp())
            .field("happiness", &self.happiness())
            .field("pokerus_status", &self.pokerus_status())
            .field("caught_data", &self.caught_data())
            .field("level", &self.level())
            .finish()
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

    pub fn set_moves(&mut self, moves: Moveset) {
        self.data[2] = moves.get(0).map_or(0, Into::into);
        self.data[3] = moves.get(1).map_or(0, Into::into);
        self.data[4] = moves.get(2).map_or(0, Into::into);
        self.data[5] = moves.get(3).map_or(0, Into::into);
    }

    pub fn set_ot_id(&mut self, ot_id: u16) {
        self.data[6..=7].copy_from_slice(&ot_id.to_be_bytes());
    }

    pub fn set_exp(&mut self, exp: u32) {
        self.data[8..=10].copy_from_slice(&exp.to_be_bytes()[1..=3]);
    }

    pub fn set_dvs(&mut self, dvs: u16) {
        self.data[21..=22].copy_from_slice(&dvs.to_be_bytes());
    }

    pub fn set_pp(&mut self, pp: [u8; 4]) {
        self.data[23..=26].copy_from_slice(&pp);
    }

    pub fn set_happiness(&mut self, happiness: u8) {
        self.data[27] = happiness;
    }

    pub fn set_level(&mut self, level: u8) {
        self.data[31] = level;
    }
}
