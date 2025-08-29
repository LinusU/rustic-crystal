use std::marker::PhantomData;

use crate::{
    game::constants::{
        pokemon_constants::{PokemonSpecies, EGG},
        text_constants::{MON_NAME_LENGTH, NAME_LENGTH},
    },
    save_state::string::PokeString,
};

pub trait MonListItem<'a> {
    const LEN: usize;

    fn new(data: &'a [u8]) -> Self;
    fn species(&self) -> PokemonSpecies;
    fn as_ref(&self) -> &[u8];
}

pub enum MonListEntry<T> {
    Mon(T, PokeString<NAME_LENGTH>, PokeString<MON_NAME_LENGTH>),
    Egg(T, PokeString<NAME_LENGTH>, PokeString<MON_NAME_LENGTH>),
}

impl<T> MonListEntry<T> {
    pub fn mon(self) -> T {
        match self {
            MonListEntry::Mon(m, ..) => m,
            MonListEntry::Egg(e, ..) => e,
        }
    }
}

pub struct MonList<'a, T: MonListItem<'a>, const N: usize> {
    data: &'a [u8],
    _marker: PhantomData<T>,
}

impl<'a, T: MonListItem<'a>, const N: usize> MonList<'a, T, N> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data[0] as usize
    }

    pub fn is_full(&self) -> bool {
        self.len() >= N
    }

    pub fn get(&'a self, index: usize) -> Option<MonListEntry<T>> {
        if index >= self.len() {
            return None;
        }

        let mon = {
            let start = 1 + N + 1 + (index * T::LEN);
            let end = start + T::LEN;
            T::new(&self.data[start..end])
        };

        let ot_name = {
            let start = 1 + N + 1 + (N * T::LEN) + (index * NAME_LENGTH);
            let end = start + NAME_LENGTH;
            PokeString::<NAME_LENGTH>::new(self.data[start..end].try_into().unwrap())
        };

        let nickname = {
            let start = 1 + N + 1 + (N * T::LEN) + (N * NAME_LENGTH) + (index * MON_NAME_LENGTH);
            let end = start + MON_NAME_LENGTH;
            PokeString::<MON_NAME_LENGTH>::new(self.data[start..end].try_into().unwrap())
        };

        if self.data[1 + index] == EGG {
            Some(MonListEntry::Egg(mon, ot_name, nickname))
        } else {
            Some(MonListEntry::Mon(mon, ot_name, nickname))
        }
    }
}

pub struct MonListMut<'a, T: MonListItem<'a>, const N: usize> {
    data: &'a mut [u8],
    _marker: PhantomData<T>,
}

impl<'a, T: MonListItem<'a>, const N: usize> MonListMut<'a, T, N> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            data,
            _marker: PhantomData,
        }
    }

    fn len(&self) -> usize {
        self.data[0] as usize
    }

    pub fn push_back(&mut self, pokemon: MonListEntry<T>) {
        assert!(self.len() < N);

        let idx = self.len();
        self.data[0] += 1;
        self.set(idx, pokemon);
        *self.species_slot_mut(idx + 1) = 0xff;
    }

    fn set(&mut self, i: usize, pokemon: MonListEntry<T>) {
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
        assert!(index <= self.len());
        &mut self.data[1 + index]
    }

    // FIXME: when generic_const_exprs is unflagged this can be &mut [u8; T::LEN]
    fn pokemon_slot_mut(&mut self, index: usize) -> &mut [u8] {
        assert!(index < self.len());
        let start = 1 + N + 1 + (index * T::LEN);
        let end = start + T::LEN;
        &mut self.data[start..end]
    }

    fn ot_name_slot_mut(&mut self, index: usize) -> &mut [u8; NAME_LENGTH] {
        assert!(index < self.len());
        let start = 1 + N + 1 + (N * T::LEN) + (index * NAME_LENGTH);
        let end = start + NAME_LENGTH;
        (&mut self.data[start..end]).try_into().unwrap()
    }

    fn nickname_slot_mut(&mut self, index: usize) -> &mut [u8; MON_NAME_LENGTH] {
        assert!(index < self.len());
        let start = 1 + N + 1 + (N * T::LEN) + (N * NAME_LENGTH) + (index * MON_NAME_LENGTH);
        let end = start + MON_NAME_LENGTH;
        (&mut self.data[start..end]).try_into().unwrap()
    }
}
