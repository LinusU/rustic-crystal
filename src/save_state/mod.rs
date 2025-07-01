use std::{
    io::{self, Read, Write},
    path::PathBuf,
};

pub mod r#box;

pub struct SaveState {
    data: [u8; 0x8000],
    rtc_zero: u64,
}

impl SaveState {
    pub fn new() -> SaveState {
        SaveState {
            data: [0; 0x8000],
            rtc_zero: 0,
        }
    }

    pub fn from_file(path: &PathBuf) -> io::Result<SaveState> {
        let mut file = std::fs::File::open(path)?;

        let mut rtc_bytes = [0; 8];
        file.read_exact(&mut rtc_bytes)?;

        let mut data = [0; 0x8000];
        file.read_exact(&mut data)?;

        Ok(SaveState {
            data,
            rtc_zero: u64::from_be_bytes(rtc_bytes),
        })
    }

    pub fn write_to_file(&self, path: &PathBuf) -> io::Result<()> {
        let mut file = std::fs::File::create(path)?;

        file.write_all(&self.rtc_zero.to_be_bytes())?;
        file.write_all(&self.data)?;

        Ok(())
    }

    pub fn byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn rtc_zero(&self) -> u64 {
        self.rtc_zero
    }

    pub fn set_rtc_zero(&mut self, value: u64) {
        self.rtc_zero = value;
    }

    pub fn current_box(&self) -> r#box::Box<'_> {
        r#box::Box::new(&self.data[0x2d10..])
    }
}
