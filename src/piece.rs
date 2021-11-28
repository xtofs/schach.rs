use druid::{Data, Lens};
use std::convert::TryFrom;
use std::fmt::{Debug, Display};

use crate::{Color, Kind};

// https://en.wikipedia.org/wiki/Chess_piece_relative_value

/// a chess piece identified by it's color and kind
/// https://en.wikipedia.org/wiki/Chess_piece
#[derive(Debug, PartialEq, Eq, Clone, Copy, Data, Lens)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("Piece").field("color", &self.color).field("kind", &self.kind).finish()
        write!(f, "{}", self.to_char())
    }
}

impl TryFrom<char> for Piece {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Piece::from_char(value)
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> Self {
        piece.to_char()
    }
}

impl Piece {
    pub fn new(color: Color, kind: Kind) -> Piece {
        Self { color, kind }
    }

    pub fn from_char(c: char) -> Result<Piece, String> {
        match c {
            'p' => Ok(Piece::new(Color::Black, Kind::Pawn)),
            'r' => Ok(Piece::new(Color::Black, Kind::Rook)),
            'n' => Ok(Piece::new(Color::Black, Kind::Knight)),
            'b' => Ok(Piece::new(Color::Black, Kind::Bishop)),
            'q' => Ok(Piece::new(Color::Black, Kind::Queen)),
            'k' => Ok(Piece::new(Color::Black, Kind::King)),
            'P' => Ok(Piece::new(Color::White, Kind::Pawn)),
            'R' => Ok(Piece::new(Color::White, Kind::Rook)),
            'N' => Ok(Piece::new(Color::White, Kind::Knight)),
            'B' => Ok(Piece::new(Color::White, Kind::Bishop)),
            'Q' => Ok(Piece::new(Color::White, Kind::Queen)),
            'K' => Ok(Piece::new(Color::White, Kind::King)),
            _ => Err("not a valid character for a chess piece".to_string()),
        }
    }

    fn to_char(&self) -> char {
        match (self.color, self.kind) {
            (Color::Black, Kind::Pawn) => 'p',
            (Color::Black, Kind::Rook) => 'r',
            (Color::Black, Kind::Knight) => 'n',
            (Color::Black, Kind::Bishop) => 'b',
            (Color::Black, Kind::Queen) => 'q',
            (Color::Black, Kind::King) => 'k',
            (Color::White, Kind::Pawn) => 'P',
            (Color::White, Kind::Rook) => 'R',
            (Color::White, Kind::Knight) => 'N',
            (Color::White, Kind::Bishop) => 'B',
            (Color::White, Kind::Queen) => 'Q',
            (Color::White, Kind::King) => 'K',
        }
    }
}
