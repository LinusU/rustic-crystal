use crate::{game::constants::move_constants::Move, rom::ROM};

const START: usize = (0x04 * 0x4000) | (0x567a & 0x3fff);

pub fn tmhm_moves() -> impl Iterator<Item = Move> {
    ROM[START..]
        .iter()
        .take_while(|&n| *n != 0)
        .map(|&n| n.into())
}
