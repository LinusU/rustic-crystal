use crate::{
    cpu::Cpu,
    game::{
        constants::{hardware_constants, serial_constants},
        macros,
        ram::{hram, sram, vram},
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

    cpu.ime = false;
    cpu.write_byte(hardware_constants::R_IF, 0);
    cpu.write_byte(hardware_constants::R_IE, 0);
    cpu.write_byte(hardware_constants::R_RP, 0);
    cpu.write_byte(hardware_constants::R_SCX, 0);
    cpu.write_byte(hardware_constants::R_SCY, 0);
    cpu.write_byte(hardware_constants::R_SB, 0);
    cpu.write_byte(hardware_constants::R_SC, 0);
    cpu.write_byte(hardware_constants::R_WX, 0);
    cpu.write_byte(hardware_constants::R_WY, 0);
    cpu.write_byte(hardware_constants::R_BGP, 0);
    cpu.write_byte(hardware_constants::R_OBP0, 0);
    cpu.write_byte(hardware_constants::R_OBP1, 0);
    cpu.write_byte(hardware_constants::R_TMA, 0);
    cpu.write_byte(hardware_constants::R_TAC, 0);

    // Start timer at 4096Hz
    cpu.write_byte(hardware_constants::R_TAC, 0b100);

    // Wait for vertical blank
    while cpu.read_byte(hardware_constants::R_LY) != (hardware_constants::LY_VBLANK + 1) {
        cpu.cycle(4);
    }

    cpu.write_byte(hardware_constants::R_LCDC, 0);

    const WRAM0_START: u16 = 0xc000; // STARTOF(WRAM0)
    const WRAM0_SIZE: u16 = 0x1000; // SIZEOF(WRAM0)

    // Clear WRAM bank 0
    for i in 0..WRAM0_SIZE {
        cpu.write_byte(WRAM0_START + i, 0);
    }

    // ld sp, wStackTop
    cpu.sp = 0xc0ff; // wStackTop

    let saved_cgb = cpu.read_byte(hram::CGB);
    let saved_system_booted = cpu.read_byte(hram::SYSTEM_BOOTED);

    const HRAM_START: u16 = 0xff80; // STARTOF(HRAM)
    const HRAM_SIZE: u16 = 0x007f; // SIZEOF(HRAM)

    // Clear HRAM
    for i in 0..HRAM_SIZE {
        cpu.write_byte(HRAM_START + i, 0);
    }

    cpu.write_byte(hram::SYSTEM_BOOTED, saved_system_booted);
    cpu.write_byte(hram::CGB, saved_cgb);

    clear_wram(cpu);

    cpu.write_byte(hardware_constants::R_SVBK, 1);

    clear_vram(cpu);
    cpu.call(0x300b); // ClearSprites
    clears_scratch(cpu);

    cpu.a = 1; // BANK(WriteOAMDMACodeToHRAM) aka BANK(GameInit)
    cpu.call(0x0010); // Bankswitch
    cpu.call(0x4031); // WriteOAMDMACodeToHRAM

    cpu.write_byte(hram::MAP_ANIMS, 0);
    cpu.write_byte(hram::SCX, 0);
    cpu.write_byte(hram::SCY, 0);
    cpu.write_byte(hardware_constants::R_JOYP, 0);

    // HBlank int enable
    cpu.write_byte(hardware_constants::R_STAT, 0x8);

    cpu.write_byte(hram::WY, 0x90);
    cpu.write_byte(hardware_constants::R_WY, 0x90);

    cpu.write_byte(hram::WX, 7);
    cpu.write_byte(hardware_constants::R_WX, 7);

    // LCD on
    // Win tilemap 1
    // Win on
    // BG/Win tiledata 0
    // BG Tilemap 0
    // OBJ 8x8
    // OBJ on
    // BG on
    cpu.write_byte(
        hardware_constants::R_LCDC,
        hardware_constants::LCDControl::default().bits(),
    );

    cpu.write_byte(
        hram::SERIAL_CONNECTION_STATUS,
        serial_constants::SerialConnectionStatus::NotEstablished as u8,
    );

    macros::farcall::farcall(cpu, 0x02, 0x5890); // InitCGBPals

    cpu.write_byte(hram::BG_MAP_ADDRESS + 1, (vram::BG_MAP_1 >> 8) as u8);
    cpu.write_byte(hram::BG_MAP_ADDRESS, (vram::BG_MAP_1 & 0xff) as u8);

    macros::farcall::farcall(cpu, 0x05, 0x4089); // StartClock

    cpu.write_byte(
        hardware_constants::MBC3_LATCH_CLOCK,
        hardware_constants::SRAM_DISABLE,
    );

    cpu.write_byte(
        hardware_constants::MBC3_SRAM_ENABLE,
        hardware_constants::SRAM_DISABLE,
    );

    cpu.call(0x2ff7); // NormalSpeed

    cpu.write_byte(hardware_constants::R_IF, 0);
    cpu.write_byte(
        hardware_constants::R_IE,
        hardware_constants::InterruptFlags::default().bits(),
    );

    cpu.ime = true;

    cpu.call(0x045a); // DelayFrame

    macros::predef::predef_call!(cpu, InitSGBBorder);

    cpu.call(0x3b4e); // InitSound

    cpu.borrow_wram_mut().set_map_music(None);

    eprintln!("Jumping to GameInit");
    cpu.pc = 0x642e; // GameInit
}

/// Wipe VRAM banks 0 and 1
fn clear_vram(cpu: &mut Cpu) {
    fn clear(cpu: &mut Cpu, bank: u8) {
        cpu.write_byte(hardware_constants::R_VBK, bank);

        const VRAM_START: u16 = 0x8000; // STARTOF(VRAM)
        const VRAM_SIZE: u16 = 0x2000; // SIZEOF(VRAM)

        for i in 0..VRAM_SIZE {
            cpu.write_byte(VRAM_START + i, 0);
        }
    }

    clear(cpu, 1);
    clear(cpu, 0);
}

/// Wipe swappable WRAM banks (1-7)
fn clear_wram(cpu: &mut Cpu) {
    const WRAMX_START: u16 = 0xd000; // STARTOF(WRAMX)
    const WRAMX_SIZE: u16 = 0x1000; // SIZEOF(WRAMX)

    for b in 1..8 {
        cpu.write_byte(hardware_constants::R_SVBK, b);

        for i in 0..WRAMX_SIZE {
            cpu.write_byte(WRAMX_START + i, 0);
        }
    }
}

/// Wipe the first 32 bytes of sScratch
fn clears_scratch(cpu: &mut Cpu) {
    cpu.a = sram::SCRATCH.0;
    cpu.call(0x2fcb); // OpenSRAM

    for i in 0..32 {
        cpu.write_byte(sram::SCRATCH.1 + i, 0);
    }

    cpu.call(0x2fe1); // CloseSRAM
}
