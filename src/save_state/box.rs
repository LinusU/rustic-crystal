use crate::{
    game::constants::{
        pokemon_constants::EGG,
        text_constants::{MON_NAME_LENGTH, NAME_LENGTH},
    },
    game_state::{
        box_mon::{BoxMonMut, BoxMonOwned, BoxMonRef},
        mon_list::MonListEntry,
    },
};

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

    pub fn get(&self, index: usize) -> Option<BoxMonRef<'_>> {
        if index >= self.len() {
            return None;
        }

        if self.data[1 + index] == 0xff {
            log::error!("List terminated before expected length");
            return None;
        }

        Some(BoxMonRef::new(&self.data[22 + (index * 32)..]))
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

    pub fn get(&mut self, index: usize) -> Option<BoxMonRef<'_>> {
        if index >= self.len() {
            return None;
        }

        if self.data[1 + index] == 0xff {
            log::error!("List terminated before expected length");
            return None;
        }

        Some(BoxMonRef::new(&self.data[22 + (index * 32)..]))
    }

    pub fn get_mut(&mut self, index: usize) -> Option<BoxMonMut<'_>> {
        if index >= self.len() {
            return None;
        }

        if self.data[1 + index] == 0xff {
            log::error!("List terminated before expected length");
            return None;
        }

        Some(BoxMonMut::new(&mut self.data[22 + (index * 32)..]))
    }

    pub fn push_back(&mut self, pokemon: MonListEntry<BoxMonRef>) {
        assert!(self.len() < 20);

        let idx = self.len();
        self.data[0] += 1;
        self.set(idx, pokemon);
        *self.species_slot_mut(idx + 1) = 0xff;
    }

    pub fn push_front(&mut self, pokemon: MonListEntry<BoxMonRef>) {
        assert!(self.len() < 20);

        self.data[0] += 1;

        if self.data[0] == 1 {
            self.data[2] = 0xff;
        } else {
            self.data.copy_within(1..21, 2);
            self.data.copy_within(22..630, 54);
            self.data.copy_within(662..871, 673);
            self.data.copy_within(882..1091, 893);
        }

        self.set(0, pokemon);
    }

    fn set(&mut self, i: usize, pokemon: MonListEntry<BoxMonRef>) {
        let (pokemon, ot_name, nickname) = match pokemon {
            MonListEntry::Egg(pokemon, ot_name, nickname) => {
                *self.species_slot_mut(i) = EGG;
                (pokemon, ot_name, nickname)
            }
            MonListEntry::Mon(pokemon, ot_name, nickname) => {
                *self.species_slot_mut(i) = pokemon.species().into();
                (pokemon, ot_name, nickname)
            }
        };

        self.pokemon_slot_mut(i).copy_from_slice(pokemon.as_ref());
        self.ot_name_slot_mut(i).copy_from_slice(ot_name.as_ref());
        self.nickname_slot_mut(i).copy_from_slice(nickname.as_ref());
    }

    fn species_slot_mut(&mut self, index: usize) -> &mut u8 {
        assert!(index < self.len());
        &mut self.data[1 + index]
    }

    fn pokemon_slot_mut(&mut self, index: usize) -> &mut [u8; BoxMonOwned::LEN] {
        assert!(index < self.len());
        let start = 22 + index * BoxMonOwned::LEN;
        let end = start + BoxMonOwned::LEN;
        (&mut self.data[start..end]).try_into().unwrap()
    }

    fn ot_name_slot_mut(&mut self, index: usize) -> &mut [u8; NAME_LENGTH] {
        assert!(index < self.len());
        let start = 662 + index * NAME_LENGTH;
        let end = start + NAME_LENGTH;
        (&mut self.data[start..end]).try_into().unwrap()
    }

    fn nickname_slot_mut(&mut self, index: usize) -> &mut [u8; MON_NAME_LENGTH] {
        assert!(index < self.len());
        let start = 882 + index * MON_NAME_LENGTH;
        let end = start + MON_NAME_LENGTH;
        (&mut self.data[start..end]).try_into().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        game::constants::pokemon_constants::PokemonSpecies,
        game_state::battle_mon::{BattleMon, BattleMonMut},
        save_state::string::PokeString,
    };

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

        let a_ot = PokeString::new([
            0x80, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);
        let b_ot = PokeString::new([
            0x81, 0x81, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);
        let c_ot = PokeString::new([
            0x82, 0x82, 0x82, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);

        let a_name = PokeString::new([
            0xa0, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);
        let b_name = PokeString::new([
            0xa1, 0xa1, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);
        let c_name = PokeString::new([
            0xa2, 0xa2, 0xa2, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50, 0x50,
        ]);

        let a = BoxMonOwned::from_battle_mon(BattleMon::new(&a_vec), 10101);
        let b = BoxMonOwned::from_battle_mon(BattleMon::new(&b_vec), 20202);
        let c = BoxMonOwned::from_battle_mon(BattleMon::new(&c_vec), 30303);

        let mut box_vec = vec![0; 1200];
        let mut r#box = BoxMut::new(&mut box_vec);

        assert_eq!(r#box.len(), 0);
        assert_eq!(r#box.get(0), None);

        r#box.push_front(MonListEntry::Mon(c.as_ref(), c_ot, c_name));

        assert_eq!(r#box.len(), 1);
        assert_eq!(r#box.get(0), Some(c.as_ref()));
        assert_eq!(r#box.get(1), None);

        r#box.push_front(MonListEntry::Mon(b.as_ref(), b_ot, b_name));

        assert_eq!(r#box.len(), 2);
        assert_eq!(r#box.get(0), Some(b.as_ref()));
        assert_eq!(r#box.get(1), Some(c.as_ref()));
        assert_eq!(r#box.get(2), None);

        r#box.push_front(MonListEntry::Mon(a.as_ref(), a_ot, a_name));

        assert_eq!(r#box.len(), 3);
        assert_eq!(r#box.get(0), Some(a.as_ref()));
        assert_eq!(r#box.get(1), Some(b.as_ref()));
        assert_eq!(r#box.get(2), Some(c.as_ref()));
        assert_eq!(r#box.get(3), None);
    }
}
