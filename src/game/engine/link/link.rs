use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{
            hardware_constants::{self, SerialTransferControl},
            serial_constants::SerialConnectionStatus,
        },
        ram::hram,
    },
};

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

pub fn wait_for_linked_friend(cpu: &mut Cpu) {
    log::debug!("wait_for_linked_friend()");

    cpu.a = cpu.borrow_wram().player_link_action();
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);

    if cpu.a != 0 {
        cpu.write_byte(
            hardware_constants::R_SB,
            SerialConnectionStatus::UsingInternalClock.into(),
        );
        cpu.cycle(12);

        cpu.write_byte(hram::SERIAL_RECEIVE, 0);

        cpu.write_byte(hardware_constants::R_SC, 0);
        cpu.cycle(4); // Handle potential serial interrupt

        cpu.write_byte(hardware_constants::R_SC, SerialTransferControl::ON.bits());
        cpu.cycle(4); // Handle potential serial interrupt

        cpu.call(0x045a); // DelayFrame
        cpu.call(0x045a); // DelayFrame
        cpu.call(0x045a); // DelayFrame
    }

    cpu.borrow_wram_mut().set_link_timeout_frames(0x02ff);

    for timeout_frames in (0..0x02ff).rev() {
        if matches!(
            SerialConnectionStatus::from(cpu.read_byte(hram::SERIAL_CONNECTION_STATUS)),
            SerialConnectionStatus::UsingInternalClock | SerialConnectionStatus::UsingExternalClock,
        ) {
            return wait_for_linked_friend_connected(cpu);
        }

        cpu.a = SerialConnectionStatus::NotEstablished.into();
        cpu.write_byte(hram::SERIAL_CONNECTION_STATUS, cpu.a);

        cpu.a = SerialConnectionStatus::UsingInternalClock.into();
        cpu.write_byte(hardware_constants::R_SB, cpu.a);

        cpu.write_byte(hram::SERIAL_RECEIVE, 0);
        cpu.write_byte(hardware_constants::R_SC, 0);
        cpu.cycle(4); // Handle potential serial interrupt

        // This write allows the player to proceed past the link receptionist's "Please wait."
        cpu.write_byte(
            hram::SERIAL_CONNECTION_STATUS,
            SerialConnectionStatus::UsingInternalClock.into(),
        );

        cpu.write_byte(hardware_constants::R_SC, SerialTransferControl::ON.bits());
        cpu.cycle(4); // Handle potential serial interrupt

        cpu.borrow_wram_mut()
            .set_link_timeout_frames(timeout_frames);

        cpu.a = SerialConnectionStatus::UsingExternalClock.into();
        cpu.write_byte(hardware_constants::R_SB, cpu.a);

        cpu.a = SerialTransferControl::CLOCK.bits();
        cpu.write_byte(hardware_constants::R_SC, cpu.a);
        cpu.cycle(4); // Handle potential serial interrupt

        cpu.a = (SerialTransferControl::ON | SerialTransferControl::CLOCK).bits();
        cpu.write_byte(hardware_constants::R_SC, cpu.a);
        cpu.cycle(4); // Handle potential serial interrupt

        cpu.call(0x045a); // DelayFrame
    }

    // Timeout
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.borrow_wram_mut().set_script_var(0);

    cpu.pc = cpu.stack_pop(); // ret
}

fn wait_for_linked_friend_connected(cpu: &mut Cpu) {
    cpu.pc = 0x5d79;

    // call LinkDataReceived
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0908); // LinkDataReceived
        cpu.pc = pc;
    }

    // call DelayFrame
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x045a); // DelayFrame
        cpu.pc = pc;
    }

    // call LinkDataReceived
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0908); // LinkDataReceived
        cpu.pc = pc;
    }

    // ld c, 50
    cpu.c = 50;
    cpu.pc += 2;
    cpu.cycle(8);

    // call DelayFrames
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0468); // DelayFrames
        cpu.pc = pc;
    }

    // ld a, $1
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wScriptVar], a
    let script_var = cpu.a;
    cpu.borrow_wram_mut().set_script_var(script_var);
    cpu.pc += 3;
    cpu.cycle(16);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
