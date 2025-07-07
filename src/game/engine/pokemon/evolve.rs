use crate::{
    cpu::{Cpu, CpuFlag},
    game::constants::{pokemon_constants::PokemonSpecies, pokemon_data_constants::EvolutionType},
    rom::ROM,
};

impl PokemonSpecies {
    fn pre_evolution(self) -> Option<PokemonSpecies> {
        const EVOS_ATTACKS_POINTERS: usize = (0x10 * 0x4000) | (0x65b1 & 0x3fff);

        for i in 0..PokemonSpecies::count() {
            let addr = u16::from_le_bytes([
                ROM[EVOS_ATTACKS_POINTERS + i * 2],
                ROM[EVOS_ATTACKS_POINTERS + i * 2 + 1],
            ]);

            let mut offset = (0x10 * 0x4000) | (addr & 0x3fff) as usize; // Adjust for ROM offset

            loop {
                let evo_type: EvolutionType = match ROM[offset] {
                    0 => break, // End of evolutions for this species
                    n => n.into(),
                };

                offset += 1;

                if evo_type == EvolutionType::Stat {
                    offset += 1; // Skip the stat comparison byte
                }

                offset += 1;

                if ROM[offset] == u8::from(self) {
                    // Found a pre-evolution that matches the current species
                    return Some(PokemonSpecies::from(i as u8 + 1)); // Convert to one-based index
                }

                offset += 1;
            }
        }

        None
    }
}

/// Find the first mon to evolve into `wCurPartySpecies`.
///
/// Return carry and the new species in `wCurPartySpecies`
/// if a pre-evolution is found.
pub fn get_pre_evolution(cpu: &mut Cpu) {
    let input = cpu.borrow_wram().cur_party_species();
    let output = input.and_then(PokemonSpecies::pre_evolution);

    log::info!("get_pre_evolution({input:?}) => {output:?}");

    cpu.set_flag(CpuFlag::C, output.is_some());

    if let Some(species) = output {
        cpu.borrow_wram_mut().set_cur_party_species(Some(species));
    }

    cpu.pc = cpu.stack_pop(); // ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_pre_evolution() {
        assert_eq!(PokemonSpecies::Squirtle.pre_evolution(), None);
        assert_eq!(PokemonSpecies::Wartortle.pre_evolution(), Some(PokemonSpecies::Squirtle));
        assert_eq!(PokemonSpecies::Blastoise.pre_evolution(), Some(PokemonSpecies::Wartortle));

        assert_eq!(PokemonSpecies::Togepi.pre_evolution(), None);

        assert_eq!(PokemonSpecies::Eevee.pre_evolution(), None);
        assert_eq!(PokemonSpecies::Jolteon.pre_evolution(), Some(PokemonSpecies::Eevee));
        assert_eq!(PokemonSpecies::Vaporeon.pre_evolution(), Some(PokemonSpecies::Eevee));
        assert_eq!(PokemonSpecies::Flareon.pre_evolution(), Some(PokemonSpecies::Eevee));
        assert_eq!(PokemonSpecies::Espeon.pre_evolution(), Some(PokemonSpecies::Eevee));
        assert_eq!(PokemonSpecies::Umbreon.pre_evolution(), Some(PokemonSpecies::Eevee));

        assert_eq!(PokemonSpecies::Mewtwo.pre_evolution(), None);
        assert_eq!(PokemonSpecies::Mew.pre_evolution(), None);
    }
}
