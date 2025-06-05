use crate::{
    cpu::{Cpu, CpuFlag},
    game::{
        constants::{hardware_constants, serial_constants},
        macros,
        ram::{hram, vram},
    },
};

pub fn start(cpu: &mut Cpu) {
    eprintln!("start()");

    cpu.write_byte(hram::CGB, 1);
    cpu.write_byte(hram::SYSTEM_BOOTED, 1);
    cpu.a = 1;

    init(cpu)
}

fn init(cpu: &mut Cpu) {
    eprintln!("init()");

    cpu.pc = 0x017d;

    // di
    cpu.ime = false;
    cpu.pc += 1;
    cpu.cycle(4);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [rIF], a
    cpu.write_byte(hardware_constants::R_IF, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rIE], a
    cpu.write_byte(hardware_constants::R_IE, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rRP], a
    cpu.write_byte(hardware_constants::R_RP, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rSCX], a
    cpu.write_byte(hardware_constants::R_SCX, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rSCY], a
    cpu.write_byte(hardware_constants::R_SCY, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rSB], a
    cpu.write_byte(hardware_constants::R_SB, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rSC], a
    cpu.write_byte(hardware_constants::R_SC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rWX], a
    cpu.write_byte(hardware_constants::R_WX, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rWY], a
    cpu.write_byte(hardware_constants::R_WY, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rBGP], a
    cpu.write_byte(hardware_constants::R_BGP, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rOBP0], a
    cpu.write_byte(hardware_constants::R_OBP0, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rOBP1], a
    cpu.write_byte(hardware_constants::R_OBP1, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rTMA], a
    cpu.write_byte(hardware_constants::R_TMA, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rTAC], a
    cpu.write_byte(hardware_constants::R_TAC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // Skip writing to wBetaTitleSequenceOpeningType, only used in pokegold-spaceworld
    cpu.write_byte(0xd000, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // Start timer at 4096Hz
    // ld a, %100
    cpu.a = 0b100;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rTAC], a
    cpu.write_byte(hardware_constants::R_TAC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    init_wait(cpu);
}

fn init_wait(cpu: &mut Cpu) {
    cpu.pc = 0x01a2;

    // ldh a, [rLY]
    cpu.a = cpu.read_byte(hardware_constants::R_LY);
    cpu.pc += 2;
    cpu.cycle(12);

    // cp LY_VBLANK + 1
    cpu.set_flag(CpuFlag::Z, cpu.a == (hardware_constants::LY_VBLANK + 1));
    cpu.set_flag(
        CpuFlag::H,
        (cpu.a & 0x0f) < ((hardware_constants::LY_VBLANK + 1) & 0x0f),
    );
    cpu.set_flag(CpuFlag::N, true);
    cpu.set_flag(CpuFlag::C, cpu.a < (hardware_constants::LY_VBLANK + 1));
    cpu.pc += 2;
    cpu.cycle(8);

    // jr nz, .wait
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return init_wait(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [rLCDC], a
    cpu.write_byte(hardware_constants::R_LCDC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // Clear WRAM bank 0
    // ld hl, STARTOF(WRAM0)
    cpu.set_hl(0xc000);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, SIZEOF(WRAM0)
    cpu.set_bc(0x1000);
    cpu.pc += 3;
    cpu.cycle(12);

    init_byte_fill(cpu);
}

fn init_byte_fill(cpu: &mut Cpu) {
    cpu.pc = 0x01b1;

    // ld [hl], 0
    cpu.write_byte(cpu.hl(), 0);
    cpu.pc += 2;
    cpu.cycle(12);

    // inc hl
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // dec bc
    cpu.set_bc(cpu.bc().wrapping_sub(1));
    cpu.pc += 1;
    cpu.cycle(8);

    // ld a, b
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);

    // or a, c
    cpu.a |= cpu.c;
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr nz, .ByteFill
    if !cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return init_byte_fill(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // ld sp, wStackTop
    cpu.sp = 0xc0ff; // wStackTop
    cpu.pc += 3;
    cpu.cycle(12);

    // Clear HRAM
    // ldh a, [hCGB]
    cpu.a = cpu.read_byte(hram::CGB);
    cpu.pc += 2;
    cpu.cycle(12);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // ldh a, [hSystemBooted]
    cpu.a = cpu.read_byte(hram::SYSTEM_BOOTED);
    cpu.pc += 2;
    cpu.cycle(12);

    // push af
    cpu.stack_push(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld hl, STARTOF(HRAM)
    cpu.set_hl(0xff80);
    cpu.pc += 3;
    cpu.cycle(12);

    // ld bc, SIZEOF(HRAM)
    cpu.set_bc(0x007f);
    cpu.pc += 3;
    cpu.cycle(12);

    // call ByteFill
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3041); // ByteFill
        cpu.pc = pc;
    }

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ldh [hSystemBooted], a
    cpu.write_byte(hram::SYSTEM_BOOTED, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // pop af
    {
        let af = cpu.stack_pop();
        cpu.set_af(af);
        cpu.pc += 1;
        cpu.cycle(12);
    }

    // ldh [hCGB], a
    cpu.write_byte(hram::CGB, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // call ClearWRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x025a); // ClearWRAM
        cpu.pc = pc;
    }

    // ld a, 1
    cpu.a = 1;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSVBK], a
    cpu.write_byte(hardware_constants::R_SVBK, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // call ClearVRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0245); // ClearVRAM
        cpu.pc = pc;
    }

    // call ClearSprites
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x300b); // ClearSprites
        cpu.pc = pc;
    }

    // call ClearsScratch
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x0270); // ClearsScratch
        cpu.pc = pc;
    }

    // aka BANK(GameInit)
    // ld a, BANK(WriteOAMDMACodeToHRAM)
    cpu.a = 0x01;
    cpu.pc += 2;
    cpu.cycle(8);

    // rst Bankswitch
    {
        cpu.pc += 1;
        let pc = cpu.pc;
        cpu.cycle(16);
        cpu.call(0x0010); // Bankswitch
        cpu.pc = pc;
    }

    // call WriteOAMDMACodeToHRAM
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x4031); // WriteOAMDMACodeToHRAM
        cpu.pc = pc;
    }

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hMapAnims], a
    cpu.write_byte(hram::MAP_ANIMS, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [hSCX], a
    cpu.write_byte(hram::SCX, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [hSCY], a
    cpu.write_byte(hram::SCY, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rJOYP], a
    cpu.write_byte(hardware_constants::R_JOYP, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // HBlank int enable
    // ld a, $8
    cpu.a = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rSTAT], a
    cpu.write_byte(hardware_constants::R_STAT, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, $90
    cpu.a = 0x90;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hWY], a
    cpu.write_byte(hram::WY, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rWY], a
    cpu.write_byte(hardware_constants::R_WY, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, 7
    cpu.a = 7;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hWX], a
    cpu.write_byte(hram::WX, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ldh [rWX], a
    cpu.write_byte(hardware_constants::R_WX, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // %11100011
    // ld a, LCDC_DEFAULT
    cpu.a = hardware_constants::LCDControl::default().bits();
    cpu.pc += 2;
    cpu.cycle(8);

    // LCD on
    // Win tilemap 1
    // Win on
    // BG/Win tiledata 0
    // BG Tilemap 0
    // OBJ 8x8
    // OBJ on
    // BG on
    // ldh [rLCDC], a
    cpu.write_byte(hardware_constants::R_LCDC, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, CONNECTION_NOT_ESTABLISHED
    cpu.a = serial_constants::SerialConnectionStatus::NotEstablished as u8;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hSerialConnectionStatus], a
    cpu.write_byte(hram::SERIAL_CONNECTION_STATUS, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // farcall InitCGBPals
    macros::farcall::farcall(cpu, 0x02, 0x5890);

    // ld a, HIGH(vBGMap1)
    cpu.a = (vram::BG_MAP_1 >> 8) as u8;
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [hBGMapAddress + 1], a
    cpu.write_byte(hram::BG_MAP_ADDRESS + 1, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // LOW(vBGMap1)
    // xor a, a
    cpu.a = (vram::BG_MAP_1 & 0xff) as u8;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [hBGMapAddress], a
    cpu.write_byte(hram::BG_MAP_ADDRESS, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // farcall StartClock
    macros::farcall::farcall(cpu, 0x05, 0x4089);

    // SRAM_DISABLE
    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [MBC3LatchClock], a
    cpu.write_byte(hardware_constants::MBC3_LATCH_CLOCK, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ld [MBC3SRamEnable], a
    cpu.write_byte(hardware_constants::MBC3_SRAM_ENABLE, cpu.a);
    cpu.pc += 3;
    cpu.cycle(16);

    // ldh a, [hCGB]
    cpu.a = cpu.read_byte(hram::CGB);
    cpu.pc += 2;
    cpu.cycle(12);

    // and a, a
    cpu.set_flag(CpuFlag::Z, cpu.a == 0);
    cpu.set_flag(CpuFlag::N, false);
    cpu.set_flag(CpuFlag::H, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // jr z, .no_double_speed
    if cpu.flag(CpuFlag::Z) {
        cpu.cycle(12);
        return init_no_double_speed(cpu);
    } else {
        cpu.pc += 2;
        cpu.cycle(8);
    }

    // call NormalSpeed
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x2ff7); // NormalSpeed
        cpu.pc = pc;
    }

    init_no_double_speed(cpu);
}

fn init_no_double_speed(cpu: &mut Cpu) {
    cpu.pc = 0x022b;

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ldh [rIF], a
    cpu.write_byte(hardware_constants::R_IF, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    // ld a, IE_DEFAULT
    cpu.a = hardware_constants::InterruptFlags::default().bits();
    cpu.pc += 2;
    cpu.cycle(8);

    // ldh [rIE], a
    cpu.write_byte(hardware_constants::R_IE, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);

    cpu.ime = true;
    cpu.pc += 1;
    cpu.cycle(4);

    // call DelayFrame
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x045a); // DelayFrame
        cpu.pc = pc;
    }

    // predef InitSGBBorder
    macros::predef::predef_call!(cpu, InitSGBBorder);

    // call InitSound
    {
        cpu.pc += 3;
        let pc = cpu.pc;
        cpu.cycle(24);
        cpu.call(0x3b4e); // InitSound
        cpu.pc = pc;
    }

    // xor a, a
    cpu.a = 0;
    cpu.set_flag(CpuFlag::Z, true);
    cpu.set_flag(CpuFlag::C, false);
    cpu.set_flag(CpuFlag::H, false);
    cpu.set_flag(CpuFlag::N, false);
    cpu.pc += 1;
    cpu.cycle(4);

    // ld [wMapMusic], a
    cpu.borrow_wram_mut().set_map_music(None);
    cpu.pc += 3;
    cpu.cycle(16);

    // jp GameInit
    cpu.cycle(16);
    cpu.call(0x642e); // GameInit
}
