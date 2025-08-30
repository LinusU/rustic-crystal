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

pub trait MonListItemMut<'a> {
    const LEN: usize;

    fn new(data: &'a mut [u8]) -> Self;
}

#[derive(Debug, PartialEq, Eq)]
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

pub struct MonListMut<'a, T: MonListItem<'a>, TMut: MonListItemMut<'a>, const N: usize> {
    data: &'a mut [u8],
    _ref: PhantomData<T>,
    _mut: PhantomData<TMut>,
}

impl<'a, T: MonListItem<'a>, TMut: MonListItemMut<'a>, const N: usize> MonListMut<'a, T, TMut, N> {
    const SPECIES_OFFSET: usize = 1;
    const POKEMON_OFFSET: usize = Self::SPECIES_OFFSET + N + 1;
    const OT_NAME_OFFSET: usize = Self::POKEMON_OFFSET + (N * T::LEN);
    const NICKNAME_OFFSET: usize = Self::OT_NAME_OFFSET + (N * NAME_LENGTH);
    const END_OFFSET: usize = Self::NICKNAME_OFFSET + (N * MON_NAME_LENGTH);

    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            data,
            _ref: PhantomData,
            _mut: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data[0] as usize
    }

    pub fn get(&'a self, index: usize) -> Option<MonListEntry<T>> {
        if index >= self.len() {
            return None;
        }

        let mon = {
            let start = Self::POKEMON_OFFSET + (index * T::LEN);
            let end = start + T::LEN;
            T::new(&self.data[start..end])
        };

        let ot_name = {
            let start = Self::OT_NAME_OFFSET + (index * NAME_LENGTH);
            let end = start + NAME_LENGTH;
            PokeString::<NAME_LENGTH>::new(self.data[start..end].try_into().unwrap())
        };

        let nickname = {
            let start = Self::NICKNAME_OFFSET + (index * MON_NAME_LENGTH);
            let end = start + MON_NAME_LENGTH;
            PokeString::<MON_NAME_LENGTH>::new(self.data[start..end].try_into().unwrap())
        };

        if self.data[1 + index] == EGG {
            Some(MonListEntry::Egg(mon, ot_name, nickname))
        } else {
            Some(MonListEntry::Mon(mon, ot_name, nickname))
        }
    }

    pub fn get_mut(&'a mut self, index: usize) -> Option<TMut> {
        if index >= self.len() {
            return None;
        }

        let start = Self::POKEMON_OFFSET + (index * T::LEN);
        let end = start + T::LEN;
        Some(TMut::new(&mut self.data[start..end]))
    }

    pub fn push_back(&mut self, pokemon: MonListEntry<T>) {
        assert!(self.len() < N);

        let idx = self.len();
        self.data[0] += 1;
        self.set(idx, pokemon);
        *self.species_slot_mut(idx + 1) = 0xff;
    }

    pub fn push_front(&mut self, pokemon: MonListEntry<T>) {
        assert!(self.len() < N);

        self.data[0] += 1;

        if self.data[0] == 1 {
            self.data[2] = 0xff;
        } else {
            self.data.copy_within(
                Self::SPECIES_OFFSET..(Self::POKEMON_OFFSET - 1),
                Self::SPECIES_OFFSET + 1,
            );
            self.data.copy_within(
                Self::POKEMON_OFFSET..(Self::OT_NAME_OFFSET - T::LEN),
                Self::POKEMON_OFFSET + T::LEN,
            );
            self.data.copy_within(
                Self::OT_NAME_OFFSET..(Self::NICKNAME_OFFSET - NAME_LENGTH),
                Self::OT_NAME_OFFSET + NAME_LENGTH,
            );
            self.data.copy_within(
                Self::NICKNAME_OFFSET..(Self::END_OFFSET - MON_NAME_LENGTH),
                Self::NICKNAME_OFFSET + MON_NAME_LENGTH,
            );
        }

        self.set(0, pokemon);
    }

    pub fn remove(&mut self, index: usize) {
        assert!(index < self.len());

        if index + 1 < self.len() {
            self.data.copy_within(
                (Self::SPECIES_OFFSET + index + 1)..Self::POKEMON_OFFSET,
                Self::SPECIES_OFFSET + index,
            );
            self.data.copy_within(
                (Self::POKEMON_OFFSET + (index + 1) * T::LEN)..Self::OT_NAME_OFFSET,
                Self::POKEMON_OFFSET + index * T::LEN,
            );
            self.data.copy_within(
                (Self::OT_NAME_OFFSET + (index + 1) * NAME_LENGTH)..Self::NICKNAME_OFFSET,
                Self::OT_NAME_OFFSET + index * NAME_LENGTH,
            );
            self.data.copy_within(
                (Self::NICKNAME_OFFSET + (index + 1) * MON_NAME_LENGTH)..Self::END_OFFSET,
                Self::NICKNAME_OFFSET + index * MON_NAME_LENGTH,
            );
        }

        self.data[0] -= 1;
        *self.species_slot_mut(self.len()) = 0xff;
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
        &mut self.data[Self::SPECIES_OFFSET + index]
    }

    // FIXME: when generic_const_exprs is unflagged this can be &mut [u8; T::LEN]
    fn pokemon_slot_mut(&mut self, index: usize) -> &mut [u8] {
        assert!(index < self.len());
        let start = Self::POKEMON_OFFSET + (index * T::LEN);
        let end = start + T::LEN;
        &mut self.data[start..end]
    }

    fn ot_name_slot_mut(&mut self, index: usize) -> &mut [u8; NAME_LENGTH] {
        assert!(index < self.len());
        let start = Self::OT_NAME_OFFSET + (index * NAME_LENGTH);
        let end = start + NAME_LENGTH;
        (&mut self.data[start..end]).try_into().unwrap()
    }

    fn nickname_slot_mut(&mut self, index: usize) -> &mut [u8; MON_NAME_LENGTH] {
        assert!(index < self.len());
        let start = Self::NICKNAME_OFFSET + (index * MON_NAME_LENGTH);
        let end = start + MON_NAME_LENGTH;
        (&mut self.data[start..end]).try_into().unwrap()
    }
}
