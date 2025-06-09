use crate::{
    cpu::Cpu,
    game::{
        macros,
        ram::{hram, vram},
    },
};

pub fn game_init(cpu: &mut Cpu) {
    cpu.pc = 0x642e;

    macros::farcall::farcall(cpu, 0x05, 0x4f1c); // TryLoadSaveData

    cpu.call(0x1fbf); // ClearWindowData
    cpu.call(0x31f3); // ClearBGPalettes
    cpu.call(0x0fc8); // ClearTilemap

    cpu.write_byte(hram::BG_MAP_ADDRESS + 1, (vram::BG_MAP_0 >> 8) as u8);
    cpu.write_byte(hram::BG_MAP_ADDRESS, (vram::BG_MAP_0 & 0xff) as u8);

    cpu.write_byte(hram::JOY_DOWN, 0);
    cpu.write_byte(hram::SCX, 0);
    cpu.write_byte(hram::SCY, 0);

    cpu.write_byte(hram::WY, 0x90);

    cpu.call(0x31f6); // WaitBGMap

    eprintln!("Jumping to IntroSequence");
    cpu.pc = 0x620b; // IntroSequence
}
