use druid::Data;
use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Data)]
pub enum Color {
    White,
    Black,
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
