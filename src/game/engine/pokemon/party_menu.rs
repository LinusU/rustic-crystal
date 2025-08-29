use crate::{
    cpu::Cpu,
    game::{data::pokemon::evos_attacks::EVOS_ATTACKS, macros},
    game_state::mon_list::MonListEntry,
};

pub fn place_party_mon_evo_stone_compatibility(cpu: &mut Cpu) {
    for i in 0..cpu.borrow_wram().party().len() {
        if let MonListEntry::Mon(mon, ..) = cpu.borrow_wram().party().get(i).unwrap() {
            let species = mon.species();
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
