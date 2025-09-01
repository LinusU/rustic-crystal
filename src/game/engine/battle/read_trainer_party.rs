use crate::{
    cpu::Cpu,
    game::{
        constants::{
            pokemon_data_constants::PARTY_LENGTH, ram_constants::MonType,
            serial_constants::LinkMode, trainer_constants::Trainer,
        },
        macros,
        ram::sram::MYSTERY_GIFT_TRAINER,
    },
    game_state::{moveset::Moveset, party_mon::PartyMonOwned},
};

pub fn read_trainer_party(cpu: &mut Cpu) {
    if cpu.borrow_wram().in_battle_tower_battle() {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    if cpu.borrow_wram().link_mode() != LinkMode::Null {
        cpu.pc = cpu.stack_pop(); // ret
        return;
    }

    let trainer = cpu.borrow_wram().other_trainer();

    log::info!("read_trainer_party({trainer:?})");

    cpu.write_byte(0xd280, 0); // wOTPartyCount
    cpu.write_byte(0xd281, 0xff); // wOTPartySpecies

    cpu.a = 0;
    cpu.set_bc((PartyMonOwned::LEN * PARTY_LENGTH) as u16);
    cpu.set_hl(0xd288); // wOTPartyMons
    cpu.call(0x3041); // ByteFill: fill bc bytes with the value of a, starting at hl

    if trainer == Trainer::Cal2 {
        cpu.a = MYSTERY_GIFT_TRAINER.0;
        cpu.call(0x2fcb); // OpenSRAM

        cpu.set_de(MYSTERY_GIFT_TRAINER.1);
        cpu.call(0x5806); // TrainerType2
        cpu.call(0x2fe1); // CloseSRAM

        return cpu.jump(0x591b); // ComputeTrainerReward
    }

    let party = trainer.party();

    for (i, mon) in party.mons.iter().enumerate() {
        let wram = cpu.borrow_wram_mut();
        wram.set_cur_party_level(mon.level);
        wram.set_cur_party_species(Some(mon.species));
        wram.set_mon_type(MonType::OtherParty);

        macros::predef::predef_call!(cpu, TryAddMonToParty);

        if let Some(item) = mon.held_item {
            cpu.borrow_wram_mut()
                .ot_party_mut()
                .get_mut(i)
                .unwrap()
                .set_item(Some(item));
        }

        if let Some(moves) = mon.moves {
            let set: Moveset = moves.into();
            let pps = set.pps();

            cpu.borrow_wram_mut()
                .ot_party_mut()
                .get_mut(i)
                .unwrap()
                .set_moves(&set)
                .set_pp(pps);
        }

        log::trace!(
            "read_trainer_party({trainer:?}) Added mon: {:?}",
            cpu.borrow_wram().ot_party().get(i)
        );
    }

    cpu.jump(0x591b) // ComputeTrainerReward
}
