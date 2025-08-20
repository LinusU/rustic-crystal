use std::{fmt::Debug, ops::Index};

use arrayvec::ArrayVec;

use crate::game::constants::{battle_constants::NUM_MOVES, move_constants::Move};

#[derive(Clone, PartialEq, Eq)]
pub struct Moveset {
    data: ArrayVec<Move, NUM_MOVES>,
}

impl Moveset {
    pub fn contains(&self, r#move: Move) -> bool {
        self.data.contains(&r#move)
    }

    pub fn get(&self, index: usize) -> Option<Move> {
        self.data.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Debug for Moveset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.data).finish()
    }
}

impl From<[u8; NUM_MOVES]> for Moveset {
    fn from(data: [u8; NUM_MOVES]) -> Self {
        Self {
            data: data
                .into_iter()
                .take_while(|&n| n != 0)
                .map(Into::into)
                .collect(),
        }
    }
}

impl Index<usize> for Moveset {
    type Output = Move;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
