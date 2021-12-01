use crate::{Color, Kind, Piece, Square};

// https://en.wikipedia.org/wiki/Algebraic_notation_(chess)

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Move {
    Move(Piece, Square, Square),
    Take(Piece, Square, Square, Kind),
    EnPassant(Piece, Square, Square),
    // TODO: Promote(Piece, Square, Square, Kind),
    Castle(Piece, Square, Square, Square, Square),
}

impl Move {
    pub fn destination(&self) -> Square {
        match self {
            Move::Move(_, _, dest) | Move::Take(_, _, dest, _) | Move::EnPassant(_, _, dest) => *dest,
            Move::Castle(_, _, dest, _, _) => *dest,
        }
    }

    pub fn player(&self) -> Color {
        match self {
            Move::Move(piece, _, _) | Move::Take(piece, _, _, _) | Move::EnPassant(piece, _, _) => piece.color,
            Move::Castle(piece, _, _, _, _) => piece.color,
        }
    }
}
