use crate::rom::ROM;
use crate::save_state::SaveState;

use std::{path::PathBuf, time};

pub struct MBC3 {
    ram: SaveState,
    pub rombank: usize,
    rambank: usize,
    selectrtc: bool,
    ram_on: bool,
    savepath: Option<PathBuf>,
    rtc_ram: [u8; 5],
    rtc_ram_latch: [u8; 5],
}

impl MBC3 {
    pub fn new() -> MBC3 {
        MBC3 {
            ram: SaveState::new(),
            rombank: 1,
            rambank: 0,
            selectrtc: false,
            ram_on: false,
            savepath: None,
            rtc_ram: [0u8; 5],
            rtc_ram_latch: [0u8; 5],
        }
    }

    fn latch_rtc_reg(&mut self) {
        self.calc_rtc_reg();
        self.rtc_ram_latch.clone_from_slice(&self.rtc_ram);
    }

    fn calc_rtc_reg(&mut self) {
        // Do not modify regs when halted
        if self.rtc_ram[4] & 0x40 == 0x40 {
            return;
        }

        let tzero = time::UNIX_EPOCH + time::Duration::from_secs(self.ram.rtc_zero());

        if self.compute_difftime() == self.ram.rtc_zero() {
            // No time has passed. Do not alter registers
            return;
        }

        let difftime = match time::SystemTime::now().duration_since(tzero) {
            Ok(n) => n.as_secs(),
            _ => 0,
        };
        self.rtc_ram[0] = (difftime % 60) as u8;
        self.rtc_ram[1] = ((difftime / 60) % 60) as u8;
        self.rtc_ram[2] = ((difftime / 3600) % 24) as u8;
        let days = difftime / (3600 * 24);
        self.rtc_ram[3] = days as u8;
        self.rtc_ram[4] = (self.rtc_ram[4] & 0xFE) | (((days >> 8) & 0x01) as u8);
        if days >= 512 {
            self.rtc_ram[4] |= 0x80;
            self.calc_rtc_zero();
        }
    }

    fn compute_difftime(&self) -> u64 {
        let mut difftime = match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
            Ok(t) => t.as_secs(),
            Err(_) => panic!("System clock is set to a time before the unix epoch (1970-01-01)"),
        };
        difftime -= self.rtc_ram[0] as u64;
        difftime -= (self.rtc_ram[1] as u64) * 60;
        difftime -= (self.rtc_ram[2] as u64) * 3600;
        let days = ((self.rtc_ram[4] as u64 & 0x1) << 8) | (self.rtc_ram[3] as u64);
        difftime -= days * 3600 * 24;
        difftime
    }

    fn calc_rtc_zero(&mut self) {
        self.ram.set_rtc_zero(self.compute_difftime());
    }
}

impl MBC3 {
    pub fn replace_ram(&mut self, ram: SaveState, path: PathBuf) {
        self.ram = ram;
        self.savepath = Some(path);
    }

    pub fn set_save_path(&mut self, path: PathBuf) {
        self.savepath = Some(path);
    }

    pub fn save_to_disk(&mut self) {
        if let Some(ref path) = self.savepath {
            self.ram.write_to_file(path).unwrap();
        }
    }

    pub fn readrom(&self, a: u16) -> u8 {
        let idx = if a < 0x4000 {
            a as usize
        } else {
            (self.rombank * 0x4000) | ((a as usize) & 0x3FFF)
        };
        *ROM.get(idx).unwrap_or(&0xFF)
    }

    pub fn readram(&self, a: u16) -> u8 {
        if !self.ram_on {
            return 0xFF;
        }

        if !self.selectrtc && self.rambank < 4 {
            self.ram
                .byte((self.rambank * 0x2000) | ((a as usize) & 0x1FFF))
        } else if self.selectrtc && self.rambank < 5 {
            self.rtc_ram_latch[self.rambank]
        } else {
            0xFF
        }
    }

    pub fn writerom(&mut self, a: u16, v: u8) {
        match a {
            0x0000..=0x1FFF => self.ram_on = (v & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                self.rombank = match v & 0x7F {
                    0 => 1,
                    n => n as usize,
                }
            }
            0x4000..=0x5FFF => {
                self.selectrtc = v & 0x8 == 0x8;
                self.rambank = (v & 0x7) as usize;
            }
            0x6000..=0x7FFF => self.latch_rtc_reg(),
            _ => panic!("Could not write to {:04X} (MBC3)", a),
        }
    }

    pub fn writeram(&mut self, a: u16, v: u8) {
        if !self.ram_on {
            return;
        }

        if !self.selectrtc && self.rambank < 4 {
            self.ram
                .set_byte((self.rambank * 0x2000) | ((a as usize) & 0x1FFF), v);
        } else if self.selectrtc && self.rambank < 5 {
            self.calc_rtc_reg();
            let vmask = match self.rambank {
                0 | 1 => 0x3F,
                2 => 0x1F,
                4 => 0xC1,
                _ => 0xFF,
            };
            self.rtc_ram[self.rambank] = v & vmask;
            self.calc_rtc_zero();
        }
    }
}
