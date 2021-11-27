use std::ops::{Add, Mul};

use crate::Square;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Dir(pub i32, pub i32);

impl Dir {
    pub(crate) fn new(x: i32, y: i32) -> Dir {
        Dir(x, y)
    }
}

impl Add<&Dir> for Square {
    type Output = Self;

    fn add(self, rhs: &Dir) -> Self::Output {
        Square {
            file: self.file + rhs.0,
            rank: self.rank + rhs.1,
        }
    }
}

impl Add<Dir> for Square {
    type Output = Self;

    fn add(self, rhs: Dir) -> Self::Output {
        Square {
            file: self.file + rhs.0,
            rank: self.rank + rhs.1,
        }
    }
}

impl Mul<i32> for Dir {
    type Output = Dir;

    fn mul(self, rhs: i32) -> Self::Output {
        Dir(self.0 * rhs, self.1 * rhs)
    }
}
