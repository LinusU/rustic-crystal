use bitflags::bitflags;

bitflags! {
    pub struct InterruptFlags: u8 {
        const VBLANK    = 1 << 0;
        const LCD_STAT  = 1 << 1;
        const TIMER     = 1 << 2;
        const SERIAL    = 1 << 3;
        const JOYPAD    = 1 << 4;
    }
}

impl Default for InterruptFlags {
    fn default() -> Self {
        Self::SERIAL | Self::TIMER | Self::LCD_STAT | Self::VBLANK
    }
}

bitflags! {
    pub struct LCDControl: u8 {
        /// 0=Off, 1=On
        const BG_PRIORITY     = 1 << 0;
        /// 0=Off, 1=On
        const SPRITES_ENABLE  = 1 << 1;
        /// 0=8x8, 1=8x16
        const SPRITE_SIZE     = 1 << 2;
        /// 0=9800-9BFF, 1=9C00-9FFF
        const BG_TILEMAP      = 1 << 3;
        /// 0=8800-97FF, 1=8000-8FFF
        const TILE_DATA       = 1 << 4;
        /// 0=Off, 1=On
        const WINDOW_ENABLE   = 1 << 5;
        /// 0=9800-9BFF, 1=9C00-9FFF
        const WINDOW_TILEMAP  = 1 << 6;
        /// 0=Off, 1=On
        const ENABLE          = 1 << 7;
    }
}

impl Default for LCDControl {
    fn default() -> Self {
        Self::ENABLE
            | Self::WINDOW_TILEMAP
            | Self::WINDOW_ENABLE
            | Self::SPRITES_ENABLE
            | Self::BG_PRIORITY
    }
}

// MBC3
pub const MBC3_SRAM_ENABLE: u16 = 0x0000;
pub const MBC3_LATCH_CLOCK: u16 = 0x6000;

pub const SRAM_DISABLE: u8 = 0x00;

/// Joypad (R/W)
pub const R_JOYP: u16 = 0xff00;

/// Serial Transfer Control (R/W)
pub const R_SC: u16 = 0xff02;

/// Timer Modulo (R/W)
pub const R_TMA: u16 = 0xff06;

/// Timer Control (R/W)
pub const R_TAC: u16 = 0xff07;

/// Serial transfer data (R/W)
pub const R_SB: u16 = 0xff01;

/// Interrupt Flag (R/W)
pub const R_IF: u16 = 0xff0f;

/// LCD Control (R/W)
pub const R_LCDC: u16 = 0xff40;

/// LCDC Status (R/W)
pub const R_STAT: u16 = 0xff41;

/// Scroll Y (R/W)
pub const R_SCY: u16 = 0xff42;

/// Scroll X (R/W)
pub const R_SCX: u16 = 0xff43;

/// LCDC Y-Coordinate (R)
pub const R_LY: u16 = 0xff44;
pub const LY_VBLANK: u8 = 144;

/// BG Palette Data (R/W) - Non CGB Mode Only
pub const R_BGP: u16 = 0xff47;

/// Object Palette 0 Data (R/W) - Non CGB Mode Only
pub const R_OBP0: u16 = 0xff48;

/// Object Palette 1 Data (R/W) - Non CGB Mode Only
pub const R_OBP1: u16 = 0xff49;

/// Window Y Position (R/W)
pub const R_WY: u16 = 0xff4a;

/// Window X Position minus 7 (R/W)
pub const R_WX: u16 = 0xff4b;

/// CGB Mode Only - VRAM Bank
pub const R_VBK: u16 = 0xff4f;

/// CGB Mode Only - Infrared Communications Port
pub const R_RP: u16 = 0xff56;

/// CGB Mode Only - WRAM Bank
pub const R_SVBK: u16 = 0xff70;

/// Interrupt Enable (R/W)
pub const R_IE: u16 = 0xffff;
