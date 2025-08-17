use crate::game::constants::{
    pokemon_constants::PokemonSpecies,
    pokemon_data_constants::{NUM_GRASSMON, NUM_WATERMON},
    ram_constants::TimeOfDay,
};

pub struct GrassWildmons {
    pub encounter_rates: [u8; 3],
    pub morn: [(u8, PokemonSpecies); NUM_GRASSMON],
    pub day: [(u8, PokemonSpecies); NUM_GRASSMON],
    pub nite: [(u8, PokemonSpecies); NUM_GRASSMON],
}

impl GrassWildmons {
    pub fn encounters(&self, time: TimeOfDay) -> &[(u8, PokemonSpecies)] {
        match time {
            TimeOfDay::Morn => &self.morn,
            TimeOfDay::Day => &self.day,
            TimeOfDay::Nite => &self.nite,
            n => panic!("Invalid time of day for wild mon encounter: {n:?}"),
        }
    }
}

pub struct WaterWildmons {
    pub encounter_rate: u8,
    pub encounters: [(u8, PokemonSpecies); NUM_WATERMON],
}

pub enum Wildmons {
    Grass(&'static GrassWildmons),
    Water(&'static WaterWildmons),
}

impl Wildmons {
    pub fn all_encounters(&self) -> Box<dyn Iterator<Item = (u8, PokemonSpecies)>> {
        match self {
            Wildmons::Grass(d) => Box::new(d.morn.iter().chain(&d.day).chain(&d.nite).copied()),
            Wildmons::Water(d) => Box::new(d.encounters.iter().copied()),
        }
    }
}
