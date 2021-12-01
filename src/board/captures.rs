use druid::Data;

use crate::{Color, Kind};
use std::fmt::Display;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Eq, Clone, Data)]
pub struct Captures([PieceCounter; 2]);

impl Default for Captures {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Index<Color> for Captures {
    type Output = PieceCounter;

    fn index(&self, color: Color) -> &Self::Output {
        &self.0[color as usize]
    }
}

impl IndexMut<Color> for Captures {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        &mut self.0[color as usize]
    }
}

impl Display for Captures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "white: {}, black: {}", self[Color::White], self[Color::Black],)
    }
}

#[derive(PartialEq, Eq, Clone, Data)]
pub struct PieceCounter([u32; 6]);

impl Default for PieceCounter {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Index<Kind> for PieceCounter {
    type Output = u32;

    fn index(&self, kind: Kind) -> &Self::Output {
        &self.0[kind as usize]
    }
}

impl IndexMut<Kind> for PieceCounter {
    fn index_mut(&mut self, kind: Kind) -> &mut Self::Output {
        &mut self.0[kind as usize]
    }
}

impl Display for PieceCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &kind in &[Kind::Pawn, Kind::Bishop, Kind::Knight, Kind::Rook, Kind::Queen] {
            if self[kind] > 0 {
                write!(f, "{:?}: {}", kind, self[kind])?;
            }
        }
        Ok(())
    }
}
