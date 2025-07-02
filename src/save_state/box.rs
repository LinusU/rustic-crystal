use crate::{
    game::constants::{
        item_constants::Item,
        move_constants::Move,
        pokemon_constants::PokemonSpecies,
        pokemon_data_constants::BASE_HAPPINESS,
        text_constants::{MON_NAME_LENGTH, NAME_LENGTH},
    },
    game_state::battle_mon::BattleMon,
    save_state::string::PokeString,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoxedMon {
    pub species: PokemonSpecies,
    pub item: Option<Item>,
    pub moves: [Option<Move>; 4],
    pub ot_id: u16,
    pub exp: u32,
    pub ev_hp: u16,
    pub ev_attack: u16,
    pub ev_defense: u16,
    pub ev_speed: u16,
    pub ev_special: u16,
    pub dvs: u16,
    pub pp: [u8; 4],
    pub friendship: u8,
    pub pokerus: u8,
    pub caught_data: [u8; 2],
    pub level: u8,
    pub ot_name: PokeString,
    pub nickname: PokeString,
}

impl BoxedMon {
    pub fn from_battle_mon(
        battle_mon: &BattleMon,
        ot_id: u16,
        ot_name: PokeString,
        nickname: PokeString,
    ) -> Self {
        assert!(ot_name.len() <= NAME_LENGTH);
        assert!(nickname.len() <= MON_NAME_LENGTH);

        Self {
            species: battle_mon.species(),
            item: battle_mon.item(),
            moves: battle_mon.moves(),
            ot_id,
            exp: battle_mon
                .species()
                .growth_rate()
                .exp_at_level(battle_mon.level()),
            ev_hp: 0,
            ev_attack: 0,
            ev_defense: 0,
            ev_speed: 0,
            ev_special: 0,
            dvs: battle_mon.dvs(),
            pp: battle_mon.pp(),
            friendship: BASE_HAPPINESS,
            pokerus: 0,
            caught_data: [0, 0],
            level: battle_mon.level(),
            ot_name,
            nickname,
        }
    }
}

pub struct Box<'a> {
    data: &'a [u8],
}

impl<'a> Box<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn capacity(&self) -> usize {
        20
    }

    pub fn len(&self) -> usize {
        self.data[0].into()
    }

    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity()
    }

    pub fn get(&self, index: usize) -> Option<BoxedMon> {
        if index >= self.len() {
            return None;
        }

        if self.data[1 + index] == 0xff {
            log::error!("List terminated before expected length");
            return None;
        }

        let offset = 22 + (index * 32);
        Some(BoxedMon {
            species: self.data[offset].into(),
            item: match self.data[offset + 1] {
                0 => None,
                n => Some(n.into()),
            },
            moves: [
                match self.data[offset + 2] {
                    0 => None,
                    n => Some(n.into()),
                },
                match self.data[offset + 3] {
                    0 => None,
                    n => Some(n.into()),
                },
                match self.data[offset + 4] {
                    0 => None,
                    n => Some(n.into()),
                },
                match self.data[offset + 5] {
                    0 => None,
                    n => Some(n.into()),
                },
            ],
            ot_id: u16::from_be_bytes([self.data[offset + 6], self.data[offset + 7]]),
            exp: u32::from_be_bytes([
                0,
                self.data[offset + 8],
                self.data[offset + 9],
                self.data[offset + 10],
            ]),
            ev_hp: u16::from_be_bytes([self.data[offset + 11], self.data[offset + 12]]),
            ev_attack: u16::from_be_bytes([self.data[offset + 13], self.data[offset + 14]]),
            ev_defense: u16::from_be_bytes([self.data[offset + 15], self.data[offset + 16]]),
            ev_speed: u16::from_be_bytes([self.data[offset + 17], self.data[offset + 18]]),
            ev_special: u16::from_be_bytes([self.data[offset + 19], self.data[offset + 20]]),
            dvs: u16::from_be_bytes([self.data[offset + 21], self.data[offset + 22]]),
            pp: [
                self.data[offset + 23],
                self.data[offset + 24],
                self.data[offset + 25],
                self.data[offset + 26],
            ],
            friendship: self.data[offset + 27],
            pokerus: self.data[offset + 28],
            caught_data: [self.data[offset + 29], self.data[offset + 30]],
            level: self.data[offset + 31],
            ot_name: PokeString::from_bytes(&self.data[662 + (index * NAME_LENGTH)..], NAME_LENGTH),
            nickname: PokeString::from_bytes(
                &self.data[882 + (index * MON_NAME_LENGTH)..],
                MON_NAME_LENGTH,
            ),
        })
    }
}

pub struct BoxMut<'a> {
    data: &'a mut [u8],
}

impl<'a> BoxMut<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data[0].into()
    }

    pub fn get(&self, index: usize) -> Option<BoxedMon> {
        Box::new(self.data).get(index)
    }

    pub fn push_front(&mut self, pokemon: &BoxedMon) {
        assert!(self.len() < 20);

        self.data[0] += 1;

        if self.data[0] == 1 {
            self.data[2] = 0xff;
        } else {
            self.data.copy_within(1..20, 2);
            self.data.copy_within(22..630, 54);
            self.data.copy_within(662..871, 673);
            self.data.copy_within(882..1091, 893);
        }

        self.set(0, pokemon);
    }

    fn set(&mut self, index: usize, pokemon: &BoxedMon) {
        assert!(index < self.len());

        self.data[1 + index] = pokemon.species.into();

        let offset = 22 + (index * 32);
        self.data[offset] = pokemon.species.into();
        self.data[offset + 1] = pokemon.item.map_or(0, Into::into);
        self.data[offset + 2] = pokemon.moves[0].map_or(0, Into::into);
        self.data[offset + 3] = pokemon.moves[1].map_or(0, Into::into);
        self.data[offset + 4] = pokemon.moves[2].map_or(0, Into::into);
        self.data[offset + 5] = pokemon.moves[3].map_or(0, Into::into);
        self.data[offset + 6..offset + 8].copy_from_slice(&pokemon.ot_id.to_be_bytes());
        self.data[offset + 8..offset + 11].copy_from_slice(&pokemon.exp.to_be_bytes()[1..]);
        self.data[offset + 11..offset + 13].copy_from_slice(&pokemon.ev_hp.to_be_bytes());
        self.data[offset + 13..offset + 15].copy_from_slice(&pokemon.ev_attack.to_be_bytes());
        self.data[offset + 15..offset + 17].copy_from_slice(&pokemon.ev_defense.to_be_bytes());
        self.data[offset + 17..offset + 19].copy_from_slice(&pokemon.ev_speed.to_be_bytes());
        self.data[offset + 19..offset + 21].copy_from_slice(&pokemon.ev_special.to_be_bytes());
        self.data[offset + 21..offset + 23].copy_from_slice(&pokemon.dvs.to_be_bytes());
        self.data[offset + 23..offset + 27].copy_from_slice(&pokemon.pp);
        self.data[offset + 27] = pokemon.friendship;
        self.data[offset + 28] = pokemon.pokerus;
        self.data[offset + 29..offset + 31].copy_from_slice(&pokemon.caught_data);
        self.data[offset + 31] = pokemon.level;

        let mut ot_name_bytes = pokemon.ot_name.iter();
        self.data[662 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[663 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[664 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[665 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[666 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[667 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[668 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[669 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[670 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[671 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);
        self.data[672 + (index * 11)] = ot_name_bytes.next().unwrap_or(0x50);

        let mut nickname_bytes = pokemon.nickname.iter();
        self.data[882 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[883 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[884 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[885 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[886 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[887 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[888 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[889 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[890 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[891 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
        self.data[892 + (index * 11)] = nickname_bytes.next().unwrap_or(0x50);
    }
}

#[cfg(test)]
mod test {
    use crate::game_state::battle_mon::BattleMonMut;

    use super::*;

    #[test]
    fn test_push_front() {
        let mut a_vec = vec![0; 100];
        let mut b_vec = vec![0; 100];
        let mut c_vec = vec![0; 100];

        let mut a = BattleMonMut::new(&mut a_vec);

        a.set_species(PokemonSpecies::Eevee);
        a.set_level(15);

        let mut b = BattleMonMut::new(&mut b_vec);

        b.set_species(PokemonSpecies::Grimer);
        b.set_level(20);

        let mut c = BattleMonMut::new(&mut c_vec);

        c.set_species(PokemonSpecies::Phanpy);
        c.set_level(18);

        let a_ot = PokeString::from_bytes(&[0x80], 1);
        let b_ot = PokeString::from_bytes(&[0x81, 0x81], 2);
        let c_ot = PokeString::from_bytes(&[0x82, 0x82, 0x82], 3);

        let a_name = PokeString::from_bytes(&[0xa0], 1);
        let b_name = PokeString::from_bytes(&[0xa1, 0xa1], 2);
        let c_name = PokeString::from_bytes(&[0xa2, 0xa2, 0xa2], 3);

        let a = BoxedMon::from_battle_mon(&BattleMon::new(&a_vec), 10101, a_ot, a_name);
        let b = BoxedMon::from_battle_mon(&BattleMon::new(&b_vec), 20202, b_ot, b_name);
        let c = BoxedMon::from_battle_mon(&BattleMon::new(&c_vec), 30303, c_ot, c_name);

        let mut box_vec = vec![0; 1200];
        let mut r#box = BoxMut::new(&mut box_vec);

        assert_eq!(r#box.len(), 0);
        assert_eq!(r#box.get(0), None);

        r#box.push_front(&c);

        assert_eq!(r#box.len(), 1);
        assert_eq!(r#box.get(0), Some(c.clone()));
        assert_eq!(r#box.get(1), None);

        r#box.push_front(&b);

        assert_eq!(r#box.len(), 2);
        assert_eq!(r#box.get(0), Some(b.clone()));
        assert_eq!(r#box.get(1), Some(c.clone()));
        assert_eq!(r#box.get(2), None);

        r#box.push_front(&a);

        assert_eq!(r#box.len(), 3);
        assert_eq!(r#box.get(0), Some(a.clone()));
        assert_eq!(r#box.get(1), Some(b.clone()));
        assert_eq!(r#box.get(2), Some(c.clone()));
        assert_eq!(r#box.get(3), None);
    }
}
