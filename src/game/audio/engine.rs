use crate::{
    cpu::Cpu,
    game::audio::{music::Music, sfx::Sfx},
};

pub fn init_sound(cpu: &mut Cpu) {
    eprintln!("init_sound()");

    cpu.mmu.sound2.stop_music();

    // Run GameBoy code as well so that everything works like normally
    // push hl
    cpu.stack_push(cpu.hl());
    cpu.pc = 0x4001;
    cpu.cycle(16);
}

pub fn play_music(cpu: &mut Cpu) {
    eprintln!("play_music(0x{:02x})", cpu.e);

    if let Some(music) = Music::from_id(cpu.e) {
        cpu.mmu.sound2.start_music(music);
    }

    // Run GameBoy code as well so that everything works like normally
    // call MusicOff
    cpu.stack_push(0x4b33);
    cpu.cycle(24);
    cpu.pc = 0x4057;
}

pub fn play_sfx(cpu: &mut Cpu) {
    eprintln!("play_sfx(0x{:02x})", cpu.e);

    if let Some(sfx) = Sfx::from_sfx_id(cpu.e) {
        cpu.mmu.sound2.play_sfx(sfx);
    }

    // Run GameBoy code as well so that everything works like normally
    // call MusicOff
    cpu.stack_push(0x4c07);
    cpu.cycle(24);
    cpu.pc = 0x4057;
}
