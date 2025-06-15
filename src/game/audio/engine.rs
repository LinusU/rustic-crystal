use crate::{
    cpu::Cpu,
    game::audio::{cry_pointers::CRIES, music::Music, sfx::Sfx},
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

pub fn play_cry(cpu: &mut Cpu) {
    let pitch = cpu.mmu.borrow_wram().cry_pitch();
    let length = cpu.mmu.borrow_wram().cry_length();

    eprintln!("play_cry({}, pitch = {pitch}, length = {length})", cpu.e);

    let sfx = CRIES[cpu.e as usize].tweaked(pitch, length);
    cpu.play_sfx(sfx);

    // Run GameBoy code as well so that everything works like normally
    // call MusicOff
    cpu.stack_push(0x4b7c);
    cpu.cycle(24);
    cpu.pc = 0x4057;
}

pub fn play_sfx(cpu: &mut Cpu) {
    eprintln!("play_sfx(0x{:02x})", cpu.e);

    if let Some(sfx) = Sfx::from_sfx_id(cpu.e) {
        cpu.play_sfx(sfx);
    }

    // Run GameBoy code as well so that everything works like normally
    // call MusicOff
    cpu.stack_push(0x4c07);
    cpu.cycle(24);
    cpu.pc = 0x4057;
}
