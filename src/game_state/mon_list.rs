use std::marker::PhantomData;

use crate::{
    game::constants::{
        pokemon_constants::EGG,
        text_constants::{MON_NAME_LENGTH, NAME_LENGTH},
    },
    save_state::string::PokeString,
};

pub trait MonListItem<'a> {
    const LEN: usize;

    fn new(data: &'a [u8]) -> Self;
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
