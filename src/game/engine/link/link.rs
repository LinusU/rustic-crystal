use crate::cpu::Cpu;

const LINK_TRADECENTER: u8 = 2;

pub fn set_bits_for_link_trade_request(cpu: &mut Cpu) {
    log::debug!("set_bits_for_link_trade_request()");

    cpu.borrow_wram_mut()
        .set_player_link_action(LINK_TRADECENTER - 1);

    cpu.borrow_wram_mut()
        .set_chosen_cable_club_room(LINK_TRADECENTER - 1);

    cpu.a = LINK_TRADECENTER - 1;

    cpu.pc = cpu.stack_pop(); // ret
}
