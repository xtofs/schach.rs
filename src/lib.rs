#![allow(incomplete_features)]
#![feature(if_let_guard)]
#![feature(iter_intersperse)]
#![feature(option_get_or_insert_default)]
#![feature(const_generics_defaults)]

#[macro_use]
extern crate lazy_static;

mod board;
mod board_widget;
mod color;
mod dir;
mod grid;
mod kind;
mod moves;
mod piece;
mod square;
mod valuation;

pub use crate::board::*;
pub use crate::board_widget::*;
pub use crate::color::*;
pub use crate::dir::*;
pub use crate::kind::*;
pub use crate::moves::*;
pub use crate::piece::*;
pub use crate::square::*;
