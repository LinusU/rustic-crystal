use crate::{
    cpu::Cpu,
    game::{
        constants::{
            pokemon_data_constants::PARTY_LENGTH, serial_constants::LinkMode,
            trainer_constants::Trainer,
        },
        ram::sram::MYSTERY_GIFT_TRAINER,
    },
    game_state::party_mon::PartyMonOwned,
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

    (cpu.a, _) = trainer.into();

    cpu.a = cpu.a.wrapping_sub(1);
    cpu.b = 0;
    cpu.c = cpu.a;

    cpu.set_hl(0x5999 + cpu.a as u16 * 2); // TrainerGroups + (trainer_class * 2)

    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.l = cpu.a;

    (_, cpu.a) = trainer.into();
    cpu.b = cpu.a;

    // Skip trainers
    'outer: loop {
        cpu.b = cpu.b.wrapping_sub(1);

        if cpu.b == 0 {
            break 'outer;
        }

        'inner: loop {
            cpu.a = cpu.read_byte(cpu.hl());
            cpu.set_hl(cpu.hl() + 1);

            if cpu.a == 0xff {
                break 'inner;
            }
        }
    }

    // Skip name
    loop {
        cpu.a = cpu.read_byte(cpu.hl());
        cpu.set_hl(cpu.hl() + 1);

        if cpu.a == 0x50 {
            break;
        }
    }

    let trainer_type = cpu.read_byte(cpu.hl());

    cpu.set_de(cpu.hl() + 1);

    match trainer_type {
        0 => cpu.call(0x57eb), // TrainerType1
        1 => cpu.call(0x5806), // TrainerType2
        2 => cpu.call(0x5871), // TrainerType3
        3 => cpu.call(0x589d), // TrainerType4
        n => panic!("Invalid trainer type: {n}"),
    }

    cpu.jump(0x591b) // ComputeTrainerReward
}
