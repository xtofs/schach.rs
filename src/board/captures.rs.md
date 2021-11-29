use crate::{Color, Kind};
use druid::Data;
use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

trait PerfectHash<const N: usize> {
    fn hash(&self) -> usize;
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Counter<T: PerfectHash<N>, const N: usize>([u32; N], PhantomData<T>);

impl<T: PerfectHash<N>, const N: usize> Default for Counter<T, N> {
    fn default() -> Self {
        Self([0; N], PhantomData::default())
    }
}

impl<T: PerfectHash<N>, const N: usize> Index<T> for Counter<T, N> {
    type Output = u32;

    fn index(&self, index: T) -> &Self::Output {
        &self.0[index.hash()]
    }
}

impl<T: PerfectHash<N>, const N: usize> IndexMut<T> for Counter<T, N> {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        &mut self.0[index.hash()]
    }
}

impl PerfectHash<6> for Kind {
    fn hash(&self) -> usize {
        match self {
            Kind::Pawn => 0,
            Kind::Bishop => 1,
            Kind::Knight => 2,
            Kind::Rook => 3,
            Kind::Queen => 4,
            Kind::King => 5,
        }
    }
}
impl Kind {
    fn enum_iter() -> std::slice::Iter<'static, Kind> {
        [Kind::Pawn, Kind::Bishop].iter()
    }
}

impl Display for Counter<Kind, 6> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(Kind::enum_iter().filter(|&&k| self[k] > 0).map(|&k| format!("{:?}:{}", k, self[k])))
            .finish()
    }
}


// ##################################################

#[derive(PartialEq, Eq, Clone)]
pub struct Captures(HashMap<Color, Counter<Kind, 6>>);

impl Default for Captures {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Index<Color> for Captures {
    type Output = Counter<Kind, 6>;

    fn index(&self, index: Color) -> &Self::Output {
        self.0.get(&index).unwrap_or_else(|| Default::default())
    }
}

impl IndexMut<Color> for Captures {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        self.0.entry(index).or_insert_with(|| Default::default())
    }
}

impl Data for Captures {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}
