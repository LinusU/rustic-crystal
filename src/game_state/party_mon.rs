use std::fmt::Debug;

use crate::{
    game::constants::{item_constants::Item, pokemon_constants::PokemonSpecies},
    game_state::{
        battle_mon::BattleMonStatus,
        box_mon::{BoxMonMut, BoxMonOwned, BoxMonRef},
        mon_list::{MonListItem, MonListItemMut},
        moveset::Moveset,
    },
    save_state::determinant_values::DeterminantValues,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyMonOwned {
    data: [u8; PartyMonOwned::LEN],
}

impl PartyMonOwned {
    pub const LEN: usize = 48;

    pub fn as_ref(&self) -> PartyMonRef<'_> {
        PartyMonRef::new(&self.data)
    }

    pub fn from_box_mon(box_mon: BoxMonRef, is_egg: bool) -> Self {
        let mut result = Self {
            data: [0; Self::LEN],
        };

        result.data[0..BoxMonOwned::LEN].copy_from_slice(box_mon.as_ref());

        let exp = box_mon.exp();
        let species = box_mon.species();
        let level = species.growth_rate().level_at_exp(exp);

        if result.level() != level {
            log::warn!("Level mismatch when converting BoxMon to PartyMon: species {:?} has level {} at exp {}, but PartyMon level is {}", box_mon.species(), level, exp, result.level());
            result.set_level(level);
        }

        // HP: (((Base + IV) * 2 + ceil(Sqrt(stat exp)) / 4) * Level) / 100 + Level + 10
        result.set_max_hp(
            (((species.base_hp() as u16 + box_mon.dvs().hp() as u16) * 2
                + (box_mon.hp_ev() as f32).sqrt().ceil() as u16 / 4)
                * level as u16)
                / 100
                + level as u16
                + 10,
        );

        // non-HP: (((Base + IV) * 2 + ceil(Sqrt(stat exp)) / 4) * Level) / 100 + 5
        result.set_attack(
            (((species.base_attack() as u16 + box_mon.dvs().attack() as u16) * 2
                + (box_mon.attack_ev() as f32).sqrt().ceil() as u16 / 4)
                * level as u16)
                / 100
                + 5,
        );

        result.set_defense(
            (((species.base_defense() as u16 + box_mon.dvs().defense() as u16) * 2
                + (box_mon.defense_ev() as f32).sqrt().ceil() as u16 / 4)
                * level as u16)
                / 100
                + 5,
        );

        result.set_speed(
            (((species.base_speed() as u16 + box_mon.dvs().speed() as u16) * 2
                + (box_mon.speed_ev() as f32).sqrt().ceil() as u16 / 4)
                * level as u16)
                / 100
                + 5,
        );

        result.set_special_attack(
            (((species.base_special_attack() as u16 + box_mon.dvs().special() as u16) * 2
                + (box_mon.special_ev() as f32).sqrt().ceil() as u16 / 4)
                * level as u16)
                / 100
                + 5,
        );

        result.set_special_defense(
            (((species.base_special_defense() as u16 + box_mon.dvs().special() as u16) * 2
                + (box_mon.special_ev() as f32).sqrt().ceil() as u16 / 4)
                * level as u16)
                / 100
                + 5,
        );

        if !is_egg {
            result.set_hp(result.max_hp());
        }

        result
    }

    pub fn level(&self) -> u8 {
        PartyMonRef::new(&self.data).level()
    }

    pub fn set_level(&mut self, level: u8) {
        PartyMonMut::new(&mut self.data).set_level(level);
    }

    pub fn set_hp(&mut self, hp: u16) {
        PartyMonMut::new(&mut self.data).set_hp(hp);
    }

    pub fn max_hp(&self) -> u16 {
        PartyMonRef::new(&self.data).max_hp()
    }

    pub fn set_max_hp(&mut self, max_hp: u16) {
        PartyMonMut::new(&mut self.data).set_max_hp(max_hp);
    }

    pub fn set_attack(&mut self, attack: u16) {
        PartyMonMut::new(&mut self.data).set_attack(attack);
    }

    pub fn set_defense(&mut self, defense: u16) {
        PartyMonMut::new(&mut self.data).set_defense(defense);
    }

    pub fn set_speed(&mut self, speed: u16) {
        PartyMonMut::new(&mut self.data).set_speed(speed);
    }

    pub fn set_special_attack(&mut self, special_attack: u16) {
        PartyMonMut::new(&mut self.data).set_special_attack(special_attack);
    }

    pub fn set_special_defense(&mut self, special_defense: u16) {
        PartyMonMut::new(&mut self.data).set_special_defense(special_defense);
    }
}

pub struct PartyMonRef<'a> {
    data: &'a [u8],
}

impl<'a> PartyMonRef<'a> {
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

    pub fn ot_id(&self) -> u16 {
        BoxMonRef::new(self.data).ot_id()
    }

    pub fn exp(&self) -> u32 {
        BoxMonRef::new(self.data).exp()
    }

    pub fn hp_ev(&self) -> u16 {
        BoxMonRef::new(self.data).hp_ev()
    }

    pub fn attack_ev(&self) -> u16 {
        BoxMonRef::new(self.data).attack_ev()
    }

    pub fn defense_ev(&self) -> u16 {
        BoxMonRef::new(self.data).defense_ev()
    }

    pub fn speed_ev(&self) -> u16 {
        BoxMonRef::new(self.data).speed_ev()
    }

    pub fn special_ev(&self) -> u16 {
        BoxMonRef::new(self.data).special_ev()
    }

    pub fn dvs(&self) -> DeterminantValues {
        BoxMonRef::new(self.data).dvs()
    }

    pub fn pp(&self) -> [u8; 4] {
        BoxMonRef::new(self.data).pp()
    }

    pub fn happiness(&self) -> u8 {
        BoxMonRef::new(self.data).happiness()
    }

    pub fn pokerus_status(&self) -> u8 {
        BoxMonRef::new(self.data).pokerus_status()
    }

    pub fn caught_data(&self) -> u16 {
        BoxMonRef::new(self.data).caught_data()
    }

    pub fn level(&self) -> u8 {
        BoxMonRef::new(self.data).level()
    }

    pub fn status(&self) -> BattleMonStatus {
        self.data[32].into()
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

    pub fn speed(&self) -> u16 {
        u16::from_be_bytes([self.data[42], self.data[43]])
    }

    pub fn spcl_atk(&self) -> u16 {
        u16::from_be_bytes([self.data[44], self.data[45]])
    }

    pub fn spcl_def(&self) -> u16 {
        u16::from_be_bytes([self.data[46], self.data[47]])
    }
}

impl<'a> Debug for PartyMonRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartyMonRef")
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
            .field("status", &self.status())
            .field("hp", &self.hp())
            .field("max_hp", &self.max_hp())
            .field("attack", &self.attack())
            .field("defense", &self.defense())
            .field("speed", &self.speed())
            .field("spcl_atk", &self.spcl_atk())
            .field("spcl_def", &self.spcl_def())
            .finish()
    }
}

impl<'a> MonListItem<'a> for PartyMonRef<'a> {
    const LEN: usize = PartyMonOwned::LEN;

    fn new(data: &'a [u8]) -> Self {
        PartyMonRef::new(data)
    }

    fn species(&self) -> PokemonSpecies {
        self.species()
    }

    fn as_ref(&self) -> &[u8] {
        self.data
    }
}

pub struct PartyMonMut<'a> {
    data: &'a mut [u8],
}

impl<'a> PartyMonMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            data: &mut data[..PartyMonOwned::LEN],
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

    pub fn set_moves(&mut self, moves: &Moveset) -> &mut Self {
        BoxMonMut::new(self.data).set_moves(moves);
        self
    }

    pub fn set_pp(&mut self, pp: [u8; 4]) {
        BoxMonMut::new(self.data).set_pp(pp);
    }

    pub fn set_happiness(&mut self, happiness: u8) {
        BoxMonMut::new(self.data).set_happiness(happiness)
    }

    pub fn set_level(&mut self, level: u8) {
        BoxMonMut::new(self.data).set_level(level);
    }

    pub fn set_hp(&mut self, hp: u16) {
        self.data[34..=35].copy_from_slice(&hp.to_be_bytes());
    }

    pub fn set_max_hp(&mut self, max_hp: u16) {
        self.data[36..=37].copy_from_slice(&max_hp.to_be_bytes());
    }

    pub fn set_attack(&mut self, attack: u16) {
        self.data[38..=39].copy_from_slice(&attack.to_be_bytes());
    }

    pub fn set_defense(&mut self, defense: u16) {
        self.data[40..=41].copy_from_slice(&defense.to_be_bytes());
    }

    pub fn set_speed(&mut self, speed: u16) {
        self.data[42..=43].copy_from_slice(&speed.to_be_bytes());
    }

    pub fn set_special_attack(&mut self, special_attack: u16) {
        self.data[44..=45].copy_from_slice(&special_attack.to_be_bytes());
    }

    pub fn set_special_defense(&mut self, special_defense: u16) {
        self.data[46..=47].copy_from_slice(&special_defense.to_be_bytes());
    }
}

impl<'a> MonListItemMut<'a> for PartyMonMut<'a> {
    const LEN: usize = PartyMonOwned::LEN;

    fn new(data: &'a mut [u8]) -> Self {
        PartyMonMut::new(data)
    }
}
