use crate::{
    cpu::Cpu,
    game::{
        constants::pokemon_constants::{PokemonSpecies, EGG},
        macros,
    },
    game_state::{box_mon::BoxMonOwned, mon_list::MonListEntry},
};

pub fn insert_pokemon_into_box(cpu: &mut Cpu) {
    log::info!(
        "insert_pokemon_into_box({} {:?})",
        cpu.borrow_wram().buffer_mon().level(),
        cpu.borrow_wram().buffer_mon().species(),
    );

    let idx = cpu.borrow_wram().cur_party_mon() as usize;
    let is_egg = cpu.borrow_wram().cur_party_species() == Some(PokemonSpecies::Unknown(EGG));
    let mon = BoxMonOwned::from_party_mon(cpu.borrow_wram().buffer_mon());
    let nickname = cpu.borrow_wram().buffer_mon_nickname();
    let ot_name = cpu.borrow_wram().buffer_mon_ot_name();

    cpu.borrow_sram_mut().current_box_mut().insert(
        idx,
        if is_egg {
            MonListEntry::Egg(mon.as_ref(), ot_name, nickname)
        } else {
            MonListEntry::Mon(mon.as_ref(), ot_name, nickname)
        },
    );

    cpu.b = idx as u8;
    macros::farcall::farcall(cpu, 0x03, 0x5cb6); // RestorePPOfDepositedPokemon

    cpu.pc = cpu.stack_pop(); // ret
}
