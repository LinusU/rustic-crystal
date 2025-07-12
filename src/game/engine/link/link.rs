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

    cpu.pc = 0x5d11;

    // ld a, [wPlayerLinkAction]
    cpu.a = cpu.borrow_wram().player_link_action();
    cpu.pc += 3;
    cpu.cycle(16);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .no_link_action
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return wait_for_linked_friend_no_link_action(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, USING_INTERNAL_CLOCK
    cpu.a = SerialConnectionStatus::UsingInternalClock.into();
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSB], a
    cpu.write_byte(hardware_constants::R_SB, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hSerialReceive], a
    cpu.write_byte(hram::SERIAL_RECEIVE, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, (0 << rSC_ON) | (0 << rSC_CLOCK)
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSC], a
    cpu.write_byte(hardware_constants::R_SC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, (1 << rSC_ON) | (0 << rSC_CLOCK)
    cpu.a = SerialTransferControl::ON.bits();
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSC], a
    cpu.write_byte(hardware_constants::R_SC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // call DelayFrame
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x045a); // DelayFrame
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

    // call DelayFrame
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x045a); // DelayFrame
        cpu.pc = pc;
    }

    wait_for_linked_friend_no_link_action(cpu);
}

fn wait_for_linked_friend_no_link_action(cpu: &mut Cpu) {
    cpu.pc = 0x5d2f;

    // ld a, $2
    cpu.a = 0x2;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wLinkTimeoutFrames + 1], a
    cpu.borrow_wram_mut().set_link_timeout_frames(0x02ff);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld a, $ff
    cpu.a = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);

    // ld [wLinkTimeoutFrames], a
    cpu.borrow_wram_mut().set_link_timeout_frames(0x02ff);
    cpu.pc += 3;
    cpu.cycle(16);

    wait_for_linked_friend_loop(cpu);
}

fn wait_for_linked_friend_loop(cpu: &mut Cpu) {
    cpu.pc = 0x5d39;

    // ldh a, [hSerialConnectionStatus]
    cpu.a = cpu.read_byte(hram::SERIAL_CONNECTION_STATUS);
    cpu.pc += 2;
    cpu.cycle(12);

    // cp USING_INTERNAL_CLOCK
    cpu.set_flag(
        CpuFlag::Z,
        cpu.a == u8::from(SerialConnectionStatus::UsingInternalClock),
    );
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(SerialConnectionStatus::UsingInternalClock) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(
        CpuFlag::C,
        cpu.a < u8::from(SerialConnectionStatus::UsingInternalClock),
    );
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .connected
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return wait_for_linked_friend_connected(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // cp USING_EXTERNAL_CLOCK
    cpu.set_flag(
        CpuFlag::Z,
        cpu.a == u8::from(SerialConnectionStatus::UsingExternalClock),
    );
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < (u8::from(SerialConnectionStatus::UsingExternalClock) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(
        CpuFlag::C,
        cpu.a < u8::from(SerialConnectionStatus::UsingExternalClock),
    );
    cpu.pc += 2;
    cpu.cycle(8);

    // jr z, .connected
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return wait_for_linked_friend_connected(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, CONNECTION_NOT_ESTABLISHED
    cpu.a = SerialConnectionStatus::NotEstablished.into();
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hSerialConnectionStatus], a
    cpu.write_byte(hram::SERIAL_CONNECTION_STATUS, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, USING_INTERNAL_CLOCK
    cpu.a = SerialConnectionStatus::UsingInternalClock.into();
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSB], a
    cpu.write_byte(hardware_constants::R_SB, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hSerialReceive], a
    cpu.write_byte(hram::SERIAL_RECEIVE, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, (0 << rSC_ON) | (0 << rSC_CLOCK)
    cpu.a = 0;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSC], a
    cpu.write_byte(hardware_constants::R_SC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, (1 << rSC_ON) | (0 << rSC_CLOCK)
    cpu.a = SerialTransferControl::ON.bits();
    cpu.pc += 2;
    cpu.cycle(8);

    // This write allows the player to proceed past the link receptionist's "Please wait."
    cpu.write_byte(
        hram::SERIAL_CONNECTION_STATUS,
        SerialConnectionStatus::UsingInternalClock.into(),
    );

    // ldh [rSC], a
    cpu.write_byte(hardware_constants::R_SC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, [wLinkTimeoutFrames]
    let mut link_timeout_frames = cpu.borrow_wram().link_timeout_frames().to_le_bytes();
    cpu.a = link_timeout_frames[0];
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wLinkTimeoutFrames], a
    link_timeout_frames[0] = cpu.a;
    cpu.borrow_wram_mut()
        .set_link_timeout_frames(u16::from_le_bytes(link_timeout_frames));
    cpu.pc += 3;
    cpu.cycle(16);

    // jr nz, .not_done
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return wait_for_linked_friend_not_done(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld a, [wLinkTimeoutFrames + 1]
    cpu.a = link_timeout_frames[1];
    cpu.pc += 3;
    cpu.cycle(16);

    // dec a
    cpu.set_flag(CpuFlag::H, (cpu.a & 0x0f) == 0x00);
    cpu.a = cpu.a.wrapping_sub(1);
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, true);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wLinkTimeoutFrames + 1], a
    link_timeout_frames[1] = cpu.a;
    cpu.borrow_wram_mut()
        .set_link_timeout_frames(u16::from_le_bytes(link_timeout_frames));
    cpu.pc += 3;
    cpu.cycle(16);

    // jr z, .done
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return wait_for_linked_friend_done(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    wait_for_linked_friend_not_done(cpu);
}

fn wait_for_linked_friend_not_done(cpu: &mut Cpu) {
    cpu.pc = 0x5d68;

    // ld a, USING_EXTERNAL_CLOCK
    cpu.a = SerialConnectionStatus::UsingExternalClock.into();
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSB], a
    cpu.write_byte(hardware_constants::R_SB, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, (0 << rSC_ON) | (1 << rSC_CLOCK)
    cpu.a = SerialTransferControl::CLOCK.bits();
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSC], a
    cpu.write_byte(hardware_constants::R_SC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, (1 << rSC_ON) | (1 << rSC_CLOCK)
    cpu.a = (SerialTransferControl::ON | SerialTransferControl::CLOCK).bits();
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSC], a
    cpu.write_byte(hardware_constants::R_SC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // call DelayFrame
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x045a); // DelayFrame
        cpu.pc = pc;
    }

    // jr .loop
    cpu.cycle(12);
    wait_for_linked_friend_loop(cpu)
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

fn wait_for_linked_friend_done(cpu: &mut Cpu) {
    cpu.pc = 0x5d8d;

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wScriptVar], a
    let script_var = cpu.a;
    cpu.borrow_wram_mut().set_script_var(script_var);
    cpu.pc += 3;
    cpu.cycle(16);

    // ret
    cpu.pc = cpu.stack_pop();
    cpu.cycle(16);
}
