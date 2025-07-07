use crate::{
    cpu::{Cpu, CpuFlag},
    game::constants::{pokemon_constants::PokemonSpecies, pokemon_data_constants::EvolutionType},
};

/// Find the first mon to evolve into `wCurPartySpecies`.
///
/// Return carry and the new species in `wCurPartySpecies`
/// if a pre-evolution is found.
pub fn get_pre_evolution(cpu: &mut Cpu) {
    log::debug!(
        "get_pre_evolution({:?})",
        cpu.borrow_wram().cur_party_species()
    );

    cpu.pc = 0x6581;

    // ld c, 0
    cpu.c = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    get_pre_evolution_loop(cpu);
}

// For each Pokemon...
fn get_pre_evolution_loop(cpu: &mut Cpu) {
    cpu.pc = 0x6583;

    // ld hl, EvosAttacksPointers
    cpu.set_hl(0x65b1);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld b, 0
    cpu.b = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // add hl, bc
    {
        let hl = cpu.hl();
        let bc = cpu.bc();
        let result = hl.wrapping_add(bc);

        cpu.set_flag(CpuFlag::H, (hl & 0x07ff) + (bc & 0x07ff) > 0x07ff);
        cpu.set_flag(CpuFlag::N, false);
        cpu.set_flag(CpuFlag::C, hl > 0xffff - bc);

        cpu.set_hl(result);
        cpu.pc += 1;
        cpu.cycle(8);
    }

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // ld h, [hl]
    cpu.h = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // ld l, a
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);

    get_pre_evolution_loop2(cpu);
}

// For each evolution...
fn get_pre_evolution_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x658d;

    // ld a, [hli]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.set_hl(cpu.hl() + 1);
    cpu.pc += 1;
    cpu.cycle(8);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // If we jump, this Pokemon does not evolve into wCurPartySpecies.
    // jr z, .no_evolve
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return get_pre_evolution_no_evolve(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // This evolution type has the extra parameter of stat comparison.
    // cp EVOLVE_STAT
    cpu.set_flag(CpuFlag::Z, cpu.a == u8::from(EvolutionType::Stat));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(EvolutionType::Stat) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < u8::from(EvolutionType::Stat));
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .not_tyrogue
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return get_pre_evolution_not_tyrogue(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    get_pre_evolution_not_tyrogue(cpu);
}

fn get_pre_evolution_not_tyrogue(cpu: &mut Cpu) {
    cpu.pc = 0x6596;

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [wCurPartySpecies]
    cpu.a = cpu.borrow_wram().cur_party_species().map_or(0, Into::into);
    cpu.pc += 3;
    cpu.cycle(16);

    // cp a, [hl]
    {
        let value = cpu.read_byte(cpu.hl());
        cpu.set_flag(CpuFlag::Z, cpu.a == value);
        cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) < (value & 0x0f));
        cpu.set_flag(CpuFlag::N, true);
        cpu.set_flag(CpuFlag::C, cpu.a < value);
    }
    cpu.pc += 1;
    cpu.cycle(8);

    // jr z, .found_preevo
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return get_pre_evolution_found_preevo(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, [hl]
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .loop2
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return get_pre_evolution_loop2(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    get_pre_evolution_no_evolve(cpu);
}

fn get_pre_evolution_no_evolve(cpu: &mut Cpu) {
    cpu.pc = 0x65a2;

    // inc c
    cpu.set_flag(CpuFlag::H, (cpu.c & 0x0f) == 0x0f);
    cpu.c = cpu.c.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, c
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);

    // cp NUM_POKEMON
    cpu.set_flag(CpuFlag::Z, cpu.a == PokemonSpecies::count() as u8);
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (PokemonSpecies::count() as u8 & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < PokemonSpecies::count() as u8);
    cpu.pc += 2;
    cpu.cycle(8);

    // jr c, .loop
    if cpu.flag(CpuFlag::C) {
        cpu.cycle(12);
        return get_pre_evolution_loop(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    log::info!(
        "get_pre_evolution({:?}) = None",
        cpu.borrow_wram().cur_party_species()
    );

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}

fn get_pre_evolution_found_preevo(cpu: &mut Cpu) {
    cpu.pc = 0x65aa;

    // inc c
    cpu.set_flag(CpuFlag::H, (cpu.c & 0x0f) == 0x0f);
    cpu.c = cpu.c.wrapping_add(1);
    cpu.set_flag(CpuFlag::Z, cpu.c == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld a, c
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);

    let input_species = cpu.borrow_wram().cur_party_species();

    // ld [wCurPartySpecies], a
    let cur_party_species = match cpu.a {
        0 => None,
        n => Some(n.into()),
    };
    cpu.borrow_wram_mut()
        .set_cur_party_species(cur_party_species);
    cpu.pc += 3;
    cpu.cycle(16);

    // scf
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::C, true);
    cpu.pc += 1;
    cpu.cycle(4);

    log::info!("get_pre_evolution({input_species:?}) = {cur_party_species:?}");

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
