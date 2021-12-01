use crate::{Kind, Piece, Square};

// https://en.wikipedia.org/wiki/Algebraic_notation_(chess)

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Move {
    pub piece: Piece,
    pub origin: Square,
    pub target: Square,
    pub kind: MoveKind,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MoveKind {
    Move(),
    Take(Kind),
    EnPassant(),
    Castle(Square, Square),
    // TODO: Promote(Piece, Square, Square, Kind),
}
impl Move {
    pub(crate) fn new_take(piece: Piece, origin: Square, target: Square, kind: Kind) -> Move {
        Move {
            piece,
            origin,
            target,
            kind: MoveKind::Take(kind),
        }
    }

    pub(crate) fn new_move(piece: Piece, origin: Square, target: Square) -> Move {
        Move {
            piece,
            origin,
            target,
            kind: MoveKind::Move(),
        }
    }

    pub(crate) fn new_en_passant(piece: Piece, origin: Square, target: Square) -> Move {
        Move {
            piece,
            origin,
            target,
            kind: MoveKind::EnPassant(),
        }
    }

    pub(crate) fn new_castle(piece: Piece, origin: Square, target: Square, rook_origin: Square, rook_target: Square) -> Move {
        Move {
            piece,
            origin,
            target,
            kind: MoveKind::Castle(rook_origin, rook_target),
        }
    }
}
