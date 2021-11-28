use crate::{Board, Kind};

trait Valuation {
    fn value(&self, board: &Board, kind: Kind) -> f32;
}

struct DefaultValuation();

impl Valuation for DefaultValuation {
    fn value(&self, board: &Board, kind: Kind) -> f32 {
        match kind {
            Kind::Pawn => 1.0,
            Kind::Knight => 3.0,
            Kind::Bishop => 3.0,
            Kind::Rook => 5.0,
            Kind::Queen => 9.0,
            Kind::King => f32::MAX,
        }
    }
}

struct SarrattValuation();

impl SarrattValuation {
    fn is_endgame(board: &Board) -> bool {
        false
    }
}

impl Valuation for SarrattValuation {
    // https://en.wikipedia.org/wiki/Chess_piece_relative_value#cite_note-1
    // pawn 2 at the start, 3+3⁄4 in the endgame;
    // knight 9+1⁄4;
    // bishop 9+3⁄4;
    // rook 15;
    // queen 23+3⁄4;
    // king as attacking piece (in the endgame) 6+1⁄2;
    // these values are divided by 3 and rounded
    fn value(&self, board: &Board, kind: Kind) -> f32 {
        let base = match kind {
            Kind::Pawn => match Self::is_endgame(board) {
                true => 2.0,
                false => 3.75,
            },
            Kind::Knight => 9.25,
            Kind::Bishop => 9.75,
            Kind::Rook => 15.0,
            Kind::Queen => 23.75,
            Kind::King => match Self::is_endgame(board) {
                true => 6.5,
                false => f32::MAX,
            },
        };
        (base / 3.0f32).round()
    }
}
