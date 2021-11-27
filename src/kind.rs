use druid::Data;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Data)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

// impl Kind {

//     #[rustfmt::skip]
//     pub fn all() -> &'static [Kind; 6] {
//         const ARRAY: [Kind; 6]= [Kind::King, Kind::Queen, Kind::Rook, Kind::Bishop, Kind::Knight, Kind::Pawn];
//         &ARRAY
//     }
// }
