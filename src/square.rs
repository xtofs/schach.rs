use std::fmt::Display;

use druid::Data;

use crate::Dir;

// square board of eight rows (called ranks) and eight columns (called files).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Data)]
pub struct Square {
    /// file = x-axis = a .. h
    pub file: i32, // x-axis, 0..7 translates to a .. h
    /// rank = y-axis = 8 .. 1
    pub rank: i32, // y-axis, 0..7 translates to 8 down to 1
}

impl Square {
    /// new square with file = x and rank = y
    pub fn new(file: i32, rank: i32) -> Square {
        Square { rank, file }
    }

    pub fn valid(&self) -> bool {
        0 <= self.file && self.file < 8 && 0 <= self.rank && self.rank < 8
    }

    #[allow(dead_code)]
    pub fn from_an(an: &str) -> Result<Square, String> {
        let chars: Vec<char> = an.chars().collect();
        if chars.len() != 2 {
            return Err(format!("'{}' doesn't match /[a-h][1-9]/", an));
        }
        let file = chars[0];
        let rank = chars[1];
        if !('a'..='h').contains(&file) {
            return Err(format!("'{}' doesn't match /[a-h][1-9]/. 1st char is {}", an, file));
        }
        if !('1'..='8').contains(&rank) {
            return Err(format!("'{}' doesn't match /[a-h][1-9]/. 2nd char is {}", an, rank));
        }
        let file = 0 + ((file as i32) - ('a' as i32)); // file - 'a'
        let rank = 8 - ((rank as i32) - ('0' as i32)); // 8 - (c - '0')

        Ok(Square { rank, file })
    }

    pub fn offset_by<'a>(&'a self, directions: &'a [Dir]) -> impl Iterator<Item = Square> + 'a {
        directions.iter().map(move |d| *self + *d).filter(|t| t.valid())
    }

    // pub fn in_direction<'a>(&'a self, dir: Dir, n: i32) -> impl Iterator<Item = Square> + 'a {
    pub fn in_direction(&self, dir: Dir, n: i32) -> impl Iterator<Item = Square> + '_ {
        (1..=n).map(move |dist| *self + (dir * dist)).filter(|t| t.valid())
    }
}

impl Display for Square {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let f = char::from((97 + self.file) as u8); // 97 = 'a'
        let r = char::from((56 - self.rank) as u8); // 56 = '8'
        write!(fmt, "{}{}", f, r)
    }
}
