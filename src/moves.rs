use crate::{piece, Color, Kind, Piece, Square};

// https://en.wikipedia.org/wiki/Algebraic_notation_(chess)

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Move {
    Move(Piece, Square, Square),
    Take(Piece, Square, Square, Kind),
    EnPassant(Piece, Square, Square),
    // TODO: Promote(Piece, Square, Square, Kind),
    // TODO: castle
}

impl Move {
    pub(crate) fn destination(&self) -> Square {
        match self {
            Move::Move(_, _, dest) | Move::Take(_, _, dest, _) | Move::EnPassant(_, _, dest) => *dest,
        }
    }

    pub(crate) fn player(&self) -> Color {
        match self {
            Move::Move(piece, _, _) | Move::Take(piece, _, _, _) | Move::EnPassant(piece, _, _) => piece.color,
        }
    }
}
