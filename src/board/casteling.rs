use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};

use druid::Data;

use crate::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Data)]
pub struct Castling {
    flags: [[bool; 2]; 2],
}

impl Castling {
    fn none() -> Self {
        Castling { flags: [[false; 2]; 2] }
    }

    pub(crate) fn from_fen(text: &str) -> Option<Self> {
        let mut result = Castling::none();
        if text == "-" {
            return Some(result);
        }
        for ch in text.chars() {
            match ch {
                'K' => result[(Color::White, Side::King)] = true,
                'Q' => result[(Color::White, Side::Queen)] = true,
                'k' => result[(Color::Black, Side::King)] = true,
                'q' => result[(Color::Black, Side::Queen)] = true,
                _ => {
                    return None;
                }
            }
        }
        Some(result)
    }
}

impl Default for Castling {
    fn default() -> Castling {
        Self { flags: [[true; 2]; 2] }
    }
}

impl Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut some = false;
        if self[(Color::White, Side::King)] {
            write!(f, "K")?;
            some = true;
        }
        if self[(Color::White, Side::Queen)] {
            write!(f, "Q")?;
            some = true;
        }
        if self[(Color::Black, Side::King)] {
            write!(f, "k")?;
            some = true;
        }
        if self[(Color::Black, Side::Queen)] {
            write!(f, "q")?;
            some = true;
        }
        if !some {
            write!(f, "-")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Data)]
pub enum Side {
    King,
    Queen,
}

impl Index<(Color, Side)> for Castling {
    type Output = bool;

    fn index(&self, index: (Color, Side)) -> &Self::Output {
        let (color, side) = index;
        &self.flags[color as usize][side as usize]
    }
}

impl IndexMut<(Color, Side)> for Castling {
    fn index_mut(&mut self, index: (Color, Side)) -> &mut Self::Output {
        let (color, side) = index;
        &mut self.flags[color as usize][side as usize]
    }
}
