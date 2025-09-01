use crate::{
    cpu::Cpu,
    game::{constants::battle_constants::TypeEffectiveness, macros},
};

/// Calculates the effectiveness of the types of the PlayerMon against the OTMon
pub fn is_the_player_mon_types_effective_against_ot_mon(cpu: &mut Cpu) {
    log::debug!(
        "is_the_player_mon_types_effective_against_ot_mon({:02x})",
        cpu.b
    );

    let species = cpu
        .borrow_wram()
        .ot_party()
        .get(cpu.b as usize)
        .unwrap()
        .mon()
        .species();

    cpu.borrow_wram_mut()
        .enemy_mon_mut()
        .set_types(species.types());

    cpu.call(0x3985); // SetPlayerTurn

    let move_type = cpu.borrow_wram().battle_mon().types().0.into();
    cpu.borrow_wram_mut().set_player_move_struct_type(move_type);
    macros::farcall::callfar(cpu, 0x0d, 0x47c8); // BattleCheckTypeMatchup
    let result = cpu.borrow_wram().type_matchup();

    if u8::from(result) > TypeEffectiveness::Effective.into() {
        return is_the_player_mon_types_effective_against_ot_mon_super_effective(cpu);
    }

    let move_type = cpu.borrow_wram().battle_mon().types().1.into();
    cpu.borrow_wram_mut().set_player_move_struct_type(move_type);
    macros::farcall::callfar(cpu, 0x0d, 0x47c8); // BattleCheckTypeMatchup
    let result = cpu.borrow_wram().type_matchup();

    if u8::from(result) > TypeEffectiveness::Effective.into() {
        return is_the_player_mon_types_effective_against_ot_mon_super_effective(cpu);
    }

    log::trace!(
        "is_the_player_mon_types_effective_against_ot_mon({:02x}) => false",
        cpu.b
    );

    cpu.pc = cpu.stack_pop(); // ret
}

fn is_the_player_mon_types_effective_against_ot_mon_super_effective(cpu: &mut Cpu) {
    log::trace!(
        "is_the_player_mon_types_effective_against_ot_mon({:02x}) => true",
        cpu.b
    );

    if cpu.borrow_wram().enemy_effectiveness_vs_player_mons(0) {
        cpu.borrow_wram_mut()
            .set_enemy_effectiveness_vs_player_mons(0, false);
    } else {
        cpu.borrow_wram_mut()
            .set_player_effectiveness_vs_enemy_mons(0, true);
    }

    cpu.pc = cpu.stack_pop(); // ret
}
