use crate::game::{audio::music::Music, constants::input_constants::JoypadButtons};

const WRAM_SIZE: usize = 0x8000;

fn fill_random(slice: &mut [u8], start: u32) {
    // Simple LCG to generate (non-cryptographic) random values
    // Each distinct invocation should use a different start value
    const A: u32 = 1103515245;
    const C: u32 = 12345;

    let mut x = start;
    for v in slice.iter_mut() {
        x = x.wrapping_mul(A).wrapping_add(C);
        *v = ((x >> 23) & 0xFF) as u8;
    }
}

pub struct GameState {
    data: [u8; WRAM_SIZE],
}

impl GameState {
    pub fn new() -> GameState {
        let mut data = [0; WRAM_SIZE];

        fill_random(&mut data, 42);

        GameState { data }
    }

    pub fn byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }
}

impl GameState {
    pub fn cry_pitch(&self) -> i16 {
        i16::from_le_bytes([self.data[0x02b0], self.data[0x02b1]])
    }

    pub fn cry_length(&self) -> u16 {
        u16::from_le_bytes([self.data[0x02b2], self.data[0x02b3]])
    }

    pub fn set_map_music(&mut self, value: Option<Music>) {
        self.data[0x02c0] = value.map_or(0, |value| value as u8);
    }

    pub fn disable_text_acceleration(&self) -> bool {
        self.data[0x02d7] != 0
    }

    pub fn set_disable_text_acceleration(&mut self, value: bool) {
        self.data[0x02d7] = if value { 1 } else { 0 };
    }

    pub fn menu_joypad(&self) -> JoypadButtons {
        JoypadButtons::from_bits(self.data[0x0f73]).unwrap()
    }

    pub fn menu_selection(&self) -> u8 {
        self.data[0x0f74]
    }

    pub fn which_index_set(&self) -> u8 {
        self.data[0x0f76]
    }

    pub fn set_which_index_set(&mut self, value: u8) {
        self.data[0x0f76] = value;
    }

    pub fn game_timer_paused(&self) -> bool {
        (self.data[0x0fbc] & 1) != 0
    }

    pub fn set_game_timer_paused(&mut self, value: bool) {
        if value {
            self.data[0x0fbc] |= 1;
        } else {
            self.data[0x0fbc] &= !1;
        }
    }

    pub fn set_no_text_scroll(&mut self, value: bool) {
        if value {
            self.data[0x0fcc] |= 1 << 4;
        } else {
            self.data[0x0fcc] &= !(1 << 4);
        }
    }

    pub fn save_file_exists(&self) -> bool {
        self.data[0x0fcd] != 0
    }

    pub fn set_save_file_exists(&mut self, value: bool) {
        self.data[0x0fcd] = if value { 1 } else { 0 };
    }
}
