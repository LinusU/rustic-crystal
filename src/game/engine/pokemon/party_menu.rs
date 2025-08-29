use crate::{
    cpu::Cpu,
    game::{data::pokemon::evos_attacks::EVOS_ATTACKS, macros},
    game_state::PartyMonSpecies,
};

pub fn place_party_mon_evo_stone_compatibility(cpu: &mut Cpu) {
    let party_count = cpu.borrow_wram().party_count();

    for i in 0..party_count {
        if cpu.borrow_wram().party_mon_species(i) != PartyMonSpecies::Egg {
            let species = cpu.borrow_wram().party_mon(i).species();
            let stone = cpu.borrow_wram().cur_item();
            let evos = EVOS_ATTACKS[u8::from(species) as usize - 1].evos;

            let str = if evos.iter().any(|e| e.is_stone_evolution(stone)) {
                0x42a3 // PlacePartyMonEvoStoneCompatibility.string_able
            } else {
                0x42a8 // PlacePartyMonEvoStoneCompatibility.string_not_able
            };

            cpu.set_hl(macros::coords::coord!(12, (1 + i) * 2));
            cpu.set_de(str);
            cpu.call(0x1078); // PlaceString
        }
    }

    cpu.pc = cpu.stack_pop(); // ret
}
