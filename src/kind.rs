use druid::Data;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Data)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}
