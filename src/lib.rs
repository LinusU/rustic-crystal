#![allow(clippy::new_without_default)]
#![recursion_limit = "192"]

pub use crate::gpu::{SCREEN_H, SCREEN_W};
pub use crate::keypad::{KeypadEvent, KeypadKey};
pub use crate::sound::{AudioPlayer, Sound};

pub mod cpu;
pub mod game;

mod game_state;
mod gpu;
mod keypad;
mod mbc3;
mod mmu;
mod rom;
mod save_state;
mod saves;
mod serial;
mod sound;
mod sound2;
mod timer;

pub type StrResult<T> = Result<T, &'static str>;
