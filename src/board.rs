use druid::{Data, Lens};
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use crate::{Color, Dir, Kind, Move, Piece, Square};

mod casteling;
use casteling::*;

#[derive(PartialEq, Eq, Clone, Data, Lens)]
/// square board of eight rows (called ranks) and eight columns (called files).
/// https://en.wikipedia.org/wiki/Chess#Setup
pub struct Board {
    pub(crate) pieces: [[Option<Piece>; 8]; 8],
    active: Color,
    castling: Castling,
    en_passant: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl Index<Square> for Board {
    type Output = Option<Piece>;

    fn index(&self, square: Square) -> &Self::Output {
        &self.pieces[square.rank as usize][square.file as usize]
    }
}

impl IndexMut<Square> for Board {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self.pieces[square.rank as usize][square.file as usize]
    }
}

impl Board {
    pub fn default() -> Self {
        Board {
            pieces: Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
                .expect("syntax error in initial board"),
            active: Color::White,
            castling: Castling::default(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 0,
        }
    }

    /// create board from Forsyth-Edwards Notation
    /// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    // Piece placement (from White's perspective). Each rank is described, starting with rank 8 and ending with rank 1; within each rank, the contents of each square are described from file "a" through file "h". Following the Standard Algebraic Notation (SAN), each piece is identified by a single letter taken from the standard English names (pawn = "P", knight = "N", bishop = "B", rook = "R", queen = "Q" and king = "K"). White pieces are designated using upper-case letters ("PNBRQK") while black pieces use lowercase ("pnbrqk"). Empty squares are noted using digits 1 through 8 (the number of empty squares), and "/" separates ranks.
    // Active color. "w" means White moves next, "b" means Black moves next.
    // Castling availability. If neither side can castle, this is "-". Otherwise, this has one or more letters: "K" (White can castle kingside), "Q" (White can castle queenside), "k" (Black can castle kingside), and/or "q" (Black can castle queenside). A move that temporarily prevents castling does not negate this notation.
    // En passant target square in algebraic notation. If there's no en passant target square, this is "-". If a pawn has just made a two-square move, this is the position "behind" the pawn. This is recorded regardless of whether there is a pawn in position to make an en passant capture.[6]
    // Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.[7]
    // Fullmove number: The number of the full move. It starts at 1, and is incremented after Black's move.
    pub fn from_fen(str: &str) -> Option<Self> {
        let mut sections = str.split_ascii_whitespace();

        let pieces = Board::parse_fen(sections.next()?)?;
        let active = Board::parse_active(sections.next()?)?;
        let castling = Castling::from_fen(sections.next()?)?;

        Some(Board {
            pieces,
            active,
            castling,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 0,
        })
    }

    // square board of eight rows (called ranks) and eight columns (called files).
    // ranks:  8..0 -> y-axis  ( j )
    // file :  a..h -> x-axis  ( i )
    pub fn piece(&self, rank: i32, file: i32) -> Option<Piece> {
        self.pieces[rank as usize][file as usize]
    }

    #[deprecated(since = "0.1.0", note = "please use `index` instead")]
    #[allow(dead_code)]
    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.pieces[square.rank as usize][square.file as usize]
    }

    fn parse_fen(str: &str) -> Option<[[Option<Piece>; 8]; 8]> {
        let mut pieces = [[None; 8]; 8];
        let mut i = 0;
        let mut j = 0;
        for c in str.chars() {
            match c {
                'p' | 'r' | 'n' | 'b' | 'q' | 'k' | 'P' | 'R' | 'N' | 'B' | 'Q' | 'K' => {
                    pieces[j][i] = Piece::from_char(c).ok();
                    i += 1;
                }
                '/' => {
                    if i != 8 {
                        eprintln!("syntax error in {}. expected '/' at {}/{} ", str, j, i)
                    }
                    j += 1;
                    i = 0;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    let v = c as usize - '0' as usize;
                    i += v;
                }
                _ => {
                    return None;
                }
            }
        }
        Some(pieces)
    }

    pub fn apply(&mut self, mv: &Move) {
        self.en_passant = None;
        match *mv {
            Move::Move(p, s, t) => {
                self[s] = None;
                self[t] = Some(p);
                if p.kind == Kind::Pawn && 2 == (s.rank - t.rank).abs() {
                    self.en_passant = Some(Square::new((s.rank + t.rank).abs() / 2, s.file));
                    dbg!(self.en_passant);
                }
            }
            Move::Take(p, s, t, _k) => {
                // self.captures.push(_k);
                self[s] = None;
                self[t] = Some(p);
                if p.kind == Kind::Pawn && 2 == (s.rank - t.rank).abs() {
                    self.en_passant = Some(Square::new((s.rank + t.rank).abs() / 2, s.file));
                    dbg!(self.en_passant);
                }
            }
        }
    }

    /// get the valid moves of the piece on given square
    pub fn get_valid_moves(&self, square: Square) -> Vec<Move> {
        if let Some(piece) = self[square] {
            match piece.kind {
                Kind::King => self.get_king_moves(square, piece),
                Kind::Queen => self.get_queen_moves(square, piece),
                Kind::Rook => self.get_rook_moves(square, piece),
                Kind::Bishop => self.get_bishop_moves(square, piece),
                Kind::Knight => self.get_knight_moves(square, piece),
                Kind::Pawn => self.get_pawn_moves(square, piece),
            }
        } else {
            vec![]
        }
    }

    fn get_knight_moves(&self, square: Square, piece: Piece) -> Vec<Move> {
        debug_assert!(piece.kind == Kind::Knight);
        let opponent = -piece.color;
        let mut result = vec![];

        for target in square.offset_by(&dirs::DIRECTION_KNIGHT) {
            match self[target] {
                Some(Piece { color, kind }) if color == opponent => {
                    // take opponents piece
                    // Piece, Square, Square, Kind),
                    let mv = Move::Take(piece, square, target, kind);
                    result.push(mv);
                }
                None => {
                    // move to empty square
                    let mv = Move::Move(piece, square, target);
                    result.push(mv);
                }
                _ => {}
            }
        }
        result
    }

    fn get_pawn_moves(&self, origin: Square, piece: Piece) -> Vec<Move> {
        debug_assert!(piece.kind == Kind::Pawn);
        let opponent = -piece.color;
        let mut result = vec![];

        let fwd = if piece.color == Color::White { -1 } else { 1 };
        let straight_fwd = Dir::new(0, fwd);

        // take diagonally
        for dest in valid_squares(&[origin + Dir(fwd, fwd), origin + Dir(fwd, -fwd)]) {
            match self[dest] {
                Some(take) if take.color == opponent => {
                    let mv = Move::Take(piece, origin, dest, take.kind);
                    result.push(mv);
                }
                _ => {}
            }
        }
        // move straight
        let starting_rank = if piece.color == Color::White { 6 } else { 1 };
        // initial move straight
        if origin.rank == starting_rank {
            let target = origin + straight_fwd * 2;
            if target.valid() && self[target].is_none() {
                result.push(Move::Move(piece, origin, target));
            }
        }
        // regular move straight
        let target = origin + straight_fwd;
        if target.valid() && self[target].is_none() {
            result.push(Move::Move(piece, origin, target));
        }

        // let final_rank = if piece.color == Color::White { 0 } else { 7 };
        // for dest in valid(&[origin + Dir(fwd, 0)]) {
        //     if self.board[dest] == None {
        //         if dest.0 != end {
        //             result.add(Move::Move(self.player, origin, Piece::Pawn, dest));
        //         } else {
        //             // https://en.wikipedia.org/wiki/Chess#Promotion
        //             for promo in [Piece::Queen, Piece::Rook, Piece::Knight, Piece::Bishop] {
        //                 result.add(Move::Promote(self.player, origin, dest, promo));
        //             }
        //         }
        //     }
        // }
        result
    }

    fn get_queen_moves(&self, square: Square, piece: Piece) -> Vec<Move> {
        debug_assert!(piece.kind == Kind::Queen);
        // let opponent = -piece.color;
        let mut result = vec![];

        for &dir in &dirs::DIRECTION_BOTH {
            self.add_moves_in_dir(&mut result, dir, piece, square, 7);
        }
        result
    }

    fn get_king_moves(&self, square: Square, piece: Piece) -> Vec<Move> {
        debug_assert!(piece.kind == Kind::King);
        // let opponent = -piece.color;
        let mut result = vec![];

        for &dir in &dirs::DIRECTION_BOTH {
            self.add_moves_in_dir(&mut result, dir, piece, square, 1);
        }
        result
    }

    fn get_rook_moves(&self, square: Square, piece: Piece) -> Vec<Move> {
        debug_assert!(piece.kind == Kind::Rook);
        let mut result = vec![];

        for &dir in &dirs::DIRECTION_RECT {
            self.add_moves_in_dir(&mut result, dir, piece, square, 7);
        }
        result
    }

    fn get_bishop_moves(&self, square: Square, piece: Piece) -> Vec<Move> {
        debug_assert!(piece.kind == Kind::Bishop);
        let mut result = vec![];

        for &dir in &dirs::DIRECTION_DIAG {
            self.add_moves_in_dir(&mut result, dir, piece, square, 7);
        }
        result
    }

    fn add_moves_in_dir(
        &self,
        result: &mut Vec<Move>,
        dir: Dir,
        piece: Piece,
        start: Square,
        n: i32,
    ) {
        let opponent = -piece.color;
        for target in start.in_direction(dir, n) {
            match self[target] {
                // opponents piece -> take it and stop moving in this direction
                Some(Piece { kind, color }) if color == opponent => {
                    result.push(Move::Take(piece, start, target, kind));
                    break;
                }
                // own piece -> stop moving in this direction
                Some(_) => {
                    break;
                }
                // move to empty square and keep going
                None => {
                    result.push(Move::Move(piece, start, target));
                }
            }
        }
    }

    fn parse_active(co: &str) -> Option<Color> {
        if co == "w" {
            Some(Color::White)
        } else if co == "b" {
            Some(Color::Black)
        } else {
            None
        }
    }
}

fn valid_squares(items: &[Square]) -> impl Iterator<Item = Square> + '_ {
    items.iter().filter(|&item| item.valid()).copied()
}

pub mod dirs {
    use super::Dir;

    const N: Dir = Dir(1, 0);
    const S: Dir = Dir(-1, 0);
    const E: Dir = Dir(0, 1);
    const W: Dir = Dir(0, -1);
    const NE: Dir = Dir(1, 1);
    const SE: Dir = Dir(-1, 1);
    const NW: Dir = Dir(1, -1);
    const SW: Dir = Dir(-1, -1);

    pub const DIRECTION_RECT: [Dir; 4] = [N, S, E, W];
    pub const DIRECTION_DIAG: [Dir; 4] = [NE, SE, NW, SW];
    pub const DIRECTION_BOTH: [Dir; 8] = [N, S, E, W, NE, SE, NW, SW];

    pub const DIRECTION_KNIGHT: [Dir; 8] = [
        Dir(2, 1),
        Dir(1, 2),
        Dir(-1, 2),
        Dir(-2, 1),
        Dir(-2, -1),
        Dir(-1, -2),
        Dir(1, -2),
        Dir(2, -1),
    ];
}
