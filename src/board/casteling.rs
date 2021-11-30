use std::ops::{Index, IndexMut};

use druid::Data;

use crate::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Data)]
pub struct Castling {
    flags: [bool; 4],
}

impl Castling {
    pub(crate) fn from_fen(text: &str) -> Option<Self> {
        let mut result = Castling { flags: [false; 4] };
        if text == "-" {
            return Some(result);
        }
        if text.contains('K') {
            result[(Color::White, Side::King)] = true;
        }
        if text.contains('Q') {
            result[(Color::White, Side::Queen)] = true;
        }
        if text.contains('k') {
            result[(Color::Black, Side::King)] = true;
        }
        if text.contains('q') {
            result[(Color::Black, Side::Queen)] = true;
        }
        Some(result)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Data)]
pub enum Side {
    King,
    Queen,
}

impl Default for Castling {
    fn default() -> Castling {
        Self { flags: [true; 4] }
    }
}

impl Index<(Color, Side)> for Castling {
    type Output = bool;

    fn index(&self, index: (Color, Side)) -> &Self::Output {
        let (color, side) = index;
        let ix = (color as usize) * 2 + (side as usize);
        &self.flags[ix]
    }
}

impl IndexMut<(Color, Side)> for Castling {
    fn index_mut(&mut self, index: (Color, Side)) -> &mut Self::Output {
        let (color, side) = index;
        let ix = (color as usize) * 2 + (side as usize);
        &mut self.flags[ix]
    }
}
