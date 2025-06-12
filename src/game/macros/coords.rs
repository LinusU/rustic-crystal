macro_rules! coord {
    ($x:expr, $y:expr) => {
        (($y as u16) * (crate::game::constants::gfx_constants::SCREEN_WIDTH as u16))
            + ($x as u16)
            + crate::game::ram::wram::TILEMAP
    };
    ($x:expr, $y:expr, $origin:expr) => {
        (($y as u16) * (crate::game::constants::gfx_constants::SCREEN_WIDTH as u16))
            + ($x as u16)
            + $origin
    };
}

pub(crate) use coord;
