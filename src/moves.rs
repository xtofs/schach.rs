use crate::{Kind, Piece, Square};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Move {
    Move(Piece, Square, Square),
    Take(Piece, Square, Square, Kind),
    // Promote(Piece, Square, Square, Kind),
    // TODO: castle
    // TODO: en passant
}

impl Move {
    pub(crate) fn target(&self) -> Square {
        match self {
            Move::Move(_, _, t) => t.clone(),
            Move::Take(_, _, t, _) => t.clone(),
            // Move::Promote(_, _, t, _) => t,
        }
    }

    pub(crate) fn is_capture(&self) -> bool {
        if let &Move::Take(_, _, _, _) = self {
            true
        } else {
            false
        }
    }
}
