use std::sync::mpsc::{Receiver, SyncSender};

use crate::game_state::GameState;
use crate::gpu::Gpu;
use crate::keypad::{Keypad, KeypadEvent};
use crate::mbc3::MBC3;
use crate::serial::{Serial, SerialCallback};
use crate::sound::Sound;
use crate::sound2::Sound2;
use crate::timer::Timer;
use crate::StrResult;

const ZRAM_SIZE: usize = 0x7F;

#[derive(PartialEq)]
enum DmaType {
    NoDMA,
    Gdma,
    Hdma,
}

#[derive(PartialEq, Copy, Clone)]
enum GbSpeed {
    Single,
    Double,
}

pub struct Mmu<'a> {
    wram: GameState,
    zram: [u8; ZRAM_SIZE],
    hdma: [u8; 4],
    pub inte: u8,
    pub intf: u8,
    pub serial: Serial<'a>,
    pub timer: Timer,
    pub keypad: Keypad,
    pub gpu: Gpu,
    pub sound: Option<Sound>,
    pub sound2: Sound2,
    hdma_status: DmaType,
    hdma_src: u16,
    hdma_dst: u16,
    hdma_len: u8,
    wrambank: usize,
    pub mbc: MBC3,
    gbspeed: GbSpeed,
    speed_switch_req: bool,
    undocumented_cgb_regs: [u8; 3], // 0xFF72, 0xFF73, 0xFF75
}

impl<'a> Mmu<'a> {
    pub fn new_cgb(
        serial_callback: Option<SerialCallback<'a>>,
        update_screen: SyncSender<Vec<u8>>,
        keypad_events: Receiver<KeypadEvent>,
    ) -> StrResult<Mmu<'a>> {
        let serial = match serial_callback {
            Some(cb) => Serial::new_with_callback(cb),
            None => Serial::new(),
        };
        let mut res = Mmu {
            wram: GameState::new(),
            zram: [0; ZRAM_SIZE],
            wrambank: 1,
            hdma: [0; 4],
            inte: 0,
            intf: 0,
            serial,
            timer: Timer::new(),
            keypad: Keypad::new(keypad_events),
            gpu: Gpu::new_cgb(update_screen),
            sound: None,
            sound2: Sound2::new(),
            mbc: MBC3::new(),
            gbspeed: GbSpeed::Single,
            speed_switch_req: false,
            hdma_src: 0,
            hdma_dst: 0,
            hdma_status: DmaType::NoDMA,
            hdma_len: 0xFF,
            undocumented_cgb_regs: [0; 3],
        };
        res.determine_mode();
        res.set_initial();
        Ok(res)
    }

    fn set_initial(&mut self) {
        self.wb(0xFF05, 0);
        self.wb(0xFF06, 0);
        self.wb(0xFF07, 0);
        self.wb(0xFF10, 0x80);
        self.wb(0xFF11, 0xBF);
        self.wb(0xFF12, 0xF3);
        self.wb(0xFF14, 0xBF);
        self.wb(0xFF16, 0x3F);
        self.wb(0xFF16, 0x3F);
        self.wb(0xFF17, 0);
        self.wb(0xFF19, 0xBF);
        self.wb(0xFF1A, 0x7F);
        self.wb(0xFF1B, 0xFF);
        self.wb(0xFF1C, 0x9F);
        self.wb(0xFF1E, 0xFF);
        self.wb(0xFF20, 0xFF);
        self.wb(0xFF21, 0);
        self.wb(0xFF22, 0);
        self.wb(0xFF23, 0xBF);
        self.wb(0xFF24, 0x77);
        self.wb(0xFF25, 0xF3);
        self.wb(0xFF26, 0xF1);
        self.wb(0xFF40, 0x91);
        self.wb(0xFF42, 0);
        self.wb(0xFF43, 0);
        self.wb(0xFF45, 0);
        self.wb(0xFF47, 0xFC);
        self.wb(0xFF48, 0xFF);
        self.wb(0xFF49, 0xFF);
        self.wb(0xFF4A, 0);
        self.wb(0xFF4B, 0);
    }

    fn determine_mode(&mut self) {
        match self.rb(0x0143) & 0x80 {
            0x80 => (),
            mode => panic!("Invalid mode: {mode}"),
        }
    }

    pub fn do_cycle(&mut self, ticks: u32) -> u32 {
        let cpudivider = match self.gbspeed {
            GbSpeed::Single => 1,
            GbSpeed::Double => 2,
        };
        let vramticks = self.perform_vramdma();
        let gputicks = ticks / cpudivider + vramticks;
        let cputicks = ticks + vramticks * cpudivider;

        self.timer.do_cycle(cputicks);
        self.intf |= self.timer.interrupt;
        self.timer.interrupt = 0;

        self.gpu.do_cycle(gputicks);
        self.intf |= self.gpu.interrupt;
        self.gpu.interrupt = 0;

        if let Some(sound) = self.sound.as_mut() {
            sound.do_cycle(gputicks);
        }

        self.intf |= self.serial.interrupt;
        self.serial.interrupt = 0;

        gputicks
    }

    pub fn rb(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc.readrom(address),
            0x8000..=0x9FFF => self.gpu.rb(address),
            0xA000..=0xBFFF => self.mbc.readram(address),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram.byte(address as usize & 0x0FFF),
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self
                .wram
                .byte((self.wrambank * 0x1000) | address as usize & 0x0FFF),
            0xFE00..=0xFE9F => self.gpu.rb(address),
            0xFF00 => self.keypad.rb(),
            0xFF01..=0xFF02 => self.serial.rb(address),
            0xFF04..=0xFF07 => self.timer.rb(address),
            0xFF0F => self.intf | 0b11100000,
            0xFF10..=0xFF3F => self.sound.as_mut().map_or(0xFF, |s| s.rb(address)),
            0xFF4D => {
                0b01111110
                    | (if self.gbspeed == GbSpeed::Double {
                        0x80
                    } else {
                        0
                    })
                    | (if self.speed_switch_req { 1 } else { 0 })
            }
            0xFF40..=0xFF4F => self.gpu.rb(address),
            0xFF51..=0xFF55 => self.hdma_read(address),
            0xFF68..=0xFF6B => self.gpu.rb(address),
            0xFF70 => self.wrambank as u8,
            0xFF72..=0xFF73 => self.undocumented_cgb_regs[address as usize - 0xFF72],
            0xFF75 => self.undocumented_cgb_regs[2] | 0b10001111,
            0xFF76..=0xFF77 => 0x00, // CGB PCM registers. Not yet implemented.
            0xFF80..=0xFFFE => self.zram[address as usize & 0x007F],
            0xFFFF => self.inte,
            _ => 0xFF,
        }
    }

    pub fn rw(&mut self, address: u16) -> u16 {
        (self.rb(address) as u16) | ((self.rb(address + 1) as u16) << 8)
    }

    pub fn wb(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.writerom(address, value),
            0x8000..=0x9FFF => self.gpu.wb(address, value),
            0xA000..=0xBFFF => self.mbc.writeram(address, value),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => {
                self.wram.set_byte(address as usize & 0x0FFF, value)
            }
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram.set_byte(
                (self.wrambank * 0x1000) | (address as usize & 0x0FFF),
                value,
            ),
            0xFE00..=0xFE9F => self.gpu.wb(address, value),
            0xFF00 => self.keypad.wb(value),
            0xFF01..=0xFF02 => self.serial.wb(address, value),
            0xFF04..=0xFF07 => self.timer.wb(address, value),
            0xFF10..=0xFF3F => self.sound.as_mut().map_or((), |s| s.wb(address, value)),
            0xFF46 => self.oamdma(value),
            0xFF4D => {
                if value & 0x1 == 0x1 {
                    self.speed_switch_req = true;
                }
            }
            0xFF40..=0xFF4F => self.gpu.wb(address, value),
            0xFF51..=0xFF55 => self.hdma_write(address, value),
            0xFF68..=0xFF6B => self.gpu.wb(address, value),
            0xFF0F => self.intf = value,
            0xFF70 => {
                self.wrambank = match value & 0x7 {
                    0 => 1,
                    n => n as usize,
                };
            }
            0xFF72..=0xFF73 => self.undocumented_cgb_regs[address as usize - 0xFF72] = value,
            0xFF75 => self.undocumented_cgb_regs[2] = value,
            0xFF80..=0xFFFE => self.zram[address as usize & 0x007F] = value,
            0xFFFF => self.inte = value,
            _ => {}
        };
    }

    pub fn ww(&mut self, address: u16, value: u16) {
        self.wb(address, (value & 0xFF) as u8);
        self.wb(address + 1, (value >> 8) as u8);
    }

    pub fn switch_speed(&mut self) {
        if self.speed_switch_req {
            if self.gbspeed == GbSpeed::Double {
                self.gbspeed = GbSpeed::Single;
            } else {
                self.gbspeed = GbSpeed::Double;
            }
        }
        self.speed_switch_req = false;
    }

    pub fn borrow_wram(&self) -> &GameState {
        &self.wram
    }

    pub fn borrow_wram_mut(&mut self) -> &mut GameState {
        &mut self.wram
    }

    fn oamdma(&mut self, value: u8) {
        let base = (value as u16) << 8;
        for i in 0..0xA0 {
            let b = self.rb(base + i);
            self.wb(0xFE00 + i, b);
        }
    }

    fn hdma_read(&self, a: u16) -> u8 {
        match a {
            0xFF51..=0xFF54 => self.hdma[(a - 0xFF51) as usize],
            0xFF55 => {
                self.hdma_len
                    | if self.hdma_status == DmaType::NoDMA {
                        0x80
                    } else {
                        0
                    }
            }
            _ => panic!("The address {a:04X} should not be handled by hdma_read"),
        }
    }

    fn hdma_write(&mut self, a: u16, v: u8) {
        match a {
            0xFF51 => self.hdma[0] = v,
            0xFF52 => self.hdma[1] = v & 0xF0,
            0xFF53 => self.hdma[2] = v & 0x1F,
            0xFF54 => self.hdma[3] = v & 0xF0,
            0xFF55 => {
                if self.hdma_status == DmaType::Hdma {
                    if v & 0x80 == 0 {
                        self.hdma_status = DmaType::NoDMA;
                    };
                    return;
                }
                let src = ((self.hdma[0] as u16) << 8) | (self.hdma[1] as u16);
                let dst = ((self.hdma[2] as u16) << 8) | (self.hdma[3] as u16) | 0x8000;
                if !(src <= 0x7FF0 || (0xA000..=0xDFF0).contains(&src)) {
                    panic!("HDMA transfer with illegal start address {src:04X}");
                }

                self.hdma_src = src;
                self.hdma_dst = dst;
                self.hdma_len = v & 0x7F;

                self.hdma_status = if v & 0x80 == 0x80 {
                    DmaType::Hdma
                } else {
                    DmaType::Gdma
                };
            }
            _ => panic!("The address {a:04X} should not be handled by hdma_write"),
        };
    }

    fn perform_vramdma(&mut self) -> u32 {
        match self.hdma_status {
            DmaType::NoDMA => 0,
            DmaType::Gdma => self.perform_gdma(),
            DmaType::Hdma => self.perform_hdma(),
        }
    }

    fn perform_hdma(&mut self) -> u32 {
        if !self.gpu.may_hdma() {
            return 0;
        }

        self.perform_vramdma_row();
        if self.hdma_len == 0x7F {
            self.hdma_status = DmaType::NoDMA;
        }

        8
    }

    fn perform_gdma(&mut self) -> u32 {
        let len = self.hdma_len as u32 + 1;
        for _i in 0..len {
            self.perform_vramdma_row();
        }

        self.hdma_status = DmaType::NoDMA;
        len * 8
    }

    fn perform_vramdma_row(&mut self) {
        let mmu_src = self.hdma_src;
        for j in 0..0x10 {
            let b: u8 = self.rb(mmu_src + j);
            self.gpu.wb(self.hdma_dst + j, b);
        }
        self.hdma_src += 0x10;
        self.hdma_dst += 0x10;

        if self.hdma_len == 0 {
            self.hdma_len = 0x7F;
        } else {
            self.hdma_len -= 1;
        }
    }
}
