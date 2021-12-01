use crate::{Color, Dir, Kind, Move, MoveKind, Piece, Square};
use druid::{Data, Lens};
use log::{info, warn};
use std::ops::{Index, IndexMut};

mod casteling;
use casteling::*;
mod captures;
use captures::*;

// https://en.wikipedia.org/wiki/Chess#Setup
// https://en.wikipedia.org/wiki/Rules_of_chess
// https://en.wikipedia.org/wiki/Chessboard
// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation

#[derive(PartialEq, Eq, Clone, Data, Lens)]
/// square board of eight rows (called ranks) and eight columns (called files).
pub struct Board {
    pieces: [[Option<Piece>; 8]; 8],
    pub active: Color,
    pub castling: Castling,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,

    pub captures: Captures,
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
            pieces: Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").expect("syntax error in initial board"),
            active: Color::White,
            castling: Default::default(),
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 0,
            captures: Default::default(),
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
        let mut board = Board::default();

        let mut sections = str.split_ascii_whitespace();
        board.pieces = Board::parse_fen(sections.next()?)?;
        board.active = Board::parse_active(sections.next()?)?;
        board.castling = Castling::from_fen(sections.next()?)?;

        Some(board)
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
                        warn!("syntax error in {}. expected '/' at {}/{} ", str, j, i)
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

    fn update_castling_elegibility(&mut self, mv: &Move) {
        if mv.piece.kind == Kind::King {
            self.castling[(self.active, Side::King)] = false;
            self.castling[(self.active, Side::Queen)] = false;
            info!("disabling casteling for {:?}", self.active);
        }
        if mv.piece.kind == Kind::Rook {
            let side = if mv.origin.file < 4 { Side::Queen } else { Side::King };
            self.castling[(self.active, side)] = false;
            info!("disabling casteling for {:?} on {:?} side", self.active, side);
        }
    }

    fn update_en_passant_elegibility(&mut self, mv: &Move) {
        let it = match mv {
            Move {
                piece: Piece { kind: Kind::Pawn, color: _ },
                origin,
                target,
                kind: MoveKind::Move(),
            }
            | Move {
                piece: Piece { kind: Kind::Pawn, color: _ },
                origin,
                target,
                kind: MoveKind::Take(_),
            } => {
                // is distance moved  2?
                if (origin.rank - target.rank).abs() == 2 {
                    // the square on the same file and rank between origin and destination
                    Some(Square::new(origin.file, (origin.rank + target.rank).abs() / 2))
                } else {
                    None
                }
            }

            _ => None,
        };
        info!("en passant elegibility {:#?}", it.map_or("-".to_string(), |sq| format!("{}", sq)));
        self.en_passant = it;
    }

    pub fn apply(&mut self, mv: &Move) {
        assert_eq!(self.active, mv.piece.color);

        let opponent = -self.active;
        let Move { piece, origin, target, kind } = *mv;
        match kind {
            MoveKind::Move() => {
                self[origin] = None;
                self[target] = Some(piece);
            }
            MoveKind::Take(_kind) => {
                assert_eq!(self[target], Some(Piece::new(opponent, _kind)));
                info!("captured {:?}", self[target]);

                self[origin] = None;
                self[target] = Some(piece);
            }
            MoveKind::EnPassant() => {
                let passed = Square::new(target.file, origin.rank);
                assert_eq!(self[passed], Some(Piece::new(opponent, Kind::Pawn)));
                info!("captured en passant {:?}", self[passed]);

                self[passed] = None;
                self[origin] = None;
                self[target] = Some(piece);
            }
            MoveKind::Castle(rook_origin, rook_target) => {
                assert_eq!(self[origin], Some(Piece::new(self.active, Kind::King)));
                assert_eq!(self[rook_origin], Some(Piece::new(self.active, Kind::Rook)));

                self[rook_target] = self[rook_origin];
                self[rook_origin] = None;
                self[origin] = None;
                self[target] = Some(piece);
            }
        }

        self.update_en_passant_elegibility(mv);
        self.update_castling_elegibility(mv);
        self.update_captures(mv.kind);

        self.halfmove_clock += 1;
        self.fullmove_number += if mv.piece.color == Color::Black { 1 } else { 0 };
        self.active = -self.active;
    }

    fn update_captures(&mut self, kind: MoveKind) {
        let opponent = -self.active;

        match kind {
            MoveKind::Take(kind) => {
                self.captures[opponent][kind] += 1;
            }
            MoveKind::EnPassant() => {
                self.captures[opponent][Kind::Pawn] += 1;
            }
            _ => {}
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
                    let mv = Move::new_take(piece, square, target, kind);
                    result.push(mv);
                }
                None => {
                    // move to empty square
                    let mv = Move::new_move(piece, square, target);
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

        fn valid_squares(items: &[Square]) -> impl Iterator<Item = Square> + '_ {
            items.iter().filter(|&item| item.valid()).copied()
        }

        // take diagonally or en-passant
        for dest in valid_squares(&[origin + Dir(1, fwd), origin + Dir(-1, fwd)]) {
            match self[dest] {
                Some(take) if take.color == opponent => {
                    result.push(Move::new_take(piece, origin, dest, take.kind));
                }
                // dest is empty and the en_passant target
                None if self.en_passant == Some(dest) => {
                    result.push(Move::new_en_passant(piece, origin, dest));
                }
                _ => {}
            }
        }

        let starting_rank = if piece.color == Color::White { 6 } else { 1 };
        // initial move straight
        if origin.rank == starting_rank {
            let target = origin + straight_fwd * 2;
            if target.valid() && self[target].is_none() {
                result.push(Move::new_move(piece, origin, target));
            }
        }
        // regular move straight
        let target = origin + straight_fwd;
        if target.valid() && self[target].is_none() {
            result.push(Move::new_move(piece, origin, target));
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
        let mut result = vec![];

        for &dir in &dirs::DIRECTION_BOTH {
            self.add_moves_in_dir(&mut result, dir, piece, square, 1);
        }

        // Castling is performed on the kingside or queenside with the rook on the same rank.[5]
        // Neither the king nor the chosen rook has previously moved.
        // There are no pieces between the king and the chosen rook.
        // One may not castle out of, through, or into check.
        if self.castling[(self.active, Side::King)] {
            self.castle(piece, square.rank, 6, 7, 5).map(|mv| result.push(mv));
        }
        if self.castling[(self.active, Side::Queen)] {
            self.castle(piece, square.rank, 2, 0, 3).map(|mv| result.push(mv));
        }
        result
    }

    fn castle(&self, piece: Piece, rank: i32, king_dst_file: i32, rook_src_file: i32, rook_dst_file: i32) -> Option<Move> {
        const KING_SRC_FILE: i32 = 4;
        debug_assert_eq!((KING_SRC_FILE - king_dst_file).abs(), 2);
        let opponent = -self.active;

        let k_src = Square::new(KING_SRC_FILE, rank);
        let k_thr = Square::new((king_dst_file + KING_SRC_FILE) / 2, rank);
        let k_dst = Square::new(king_dst_file, rank);
        let r_src = Square::new(rook_src_file, rank);
        let r_dst = Square::new(rook_dst_file, rank);
        if !self.is_under_attack(k_src, opponent) && !self.is_under_attack(k_dst, opponent) && !self.is_under_attack(k_thr, opponent) {
            let mc = Move::new_castle(piece, k_src, k_dst, r_src, r_dst);
            info!("casteling {:?} possible", mc);
            return Some(mc);
        }
        None
    }

    fn is_under_attack(&self, _square: Square, _by: Color) -> bool {
        false
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

    fn add_moves_in_dir(&self, result: &mut Vec<Move>, dir: Dir, piece: Piece, start: Square, n: i32) {
        let opponent = -piece.color;
        for target in start.in_direction(dir, n) {
            match self[target] {
                // opponents piece -> take it and stop moving in this direction
                Some(Piece { kind, color }) if color == opponent => {
                    result.push(Move::new_take(piece, start, target, kind));
                    break;
                }
                // own piece -> stop moving in this direction
                Some(_) => {
                    break;
                }
                // move to empty square and keep going
                None => {
                    result.push(Move::new_move(piece, start, target));
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

    pub const DIRECTION_KNIGHT: [Dir; 8] = [Dir(2, 1), Dir(1, 2), Dir(-1, 2), Dir(-2, 1), Dir(-2, -1), Dir(-1, -2), Dir(1, -2), Dir(2, -1)];
}
