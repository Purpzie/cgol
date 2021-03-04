//! Contains the [`Cell`] enum.

#[macro_use]
mod macros;

use core::ops::*;
pub use Cell::{Alive, Dead};

/// A "cell" which can be alive or dead.
///
/// It can be cast into any integer type using the [`as`] keyword.
/// [`Alive`] becomes `1`, and [`Dead`] becomes `0`.
///
/// [`as`]: https://doc.rust-lang.org/std/keyword.as.html
///
/// Internally, it is represented as a [`u8`].
///
/// ## Notable Implementations
/// - Using the unary operator `!` will invert the cell.
/// - Its [default](Default) is [`Dead`].
/// - [From] and [into](Into) [`bool`].
/// - [From] all integer types. Returns [`Alive`] if `> 0`, or [`Dead`] otherwise.
/// - [Into] all integer types. Works the same as a cast.
/// - Every mathematical operator for every integer type, as long as the
///   integer is on the left. The cell will be cast before the operation.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Default for Cell {
    #[inline]
    fn default() -> Self {
        Dead
    }
}

impl From<bool> for Cell {
    #[inline]
    fn from(b: bool) -> Cell {
        if b {
            Alive
        } else {
            Dead
        }
    }
}

impl From<Cell> for bool {
    #[inline]
    fn from(c: Cell) -> bool {
        c == Alive
    }
}

impl Not for Cell {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Alive => Dead,
            _ => Alive,
        }
    }
}

// see ./macros.rs
integers! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
}

forward_unop!(impl Not::not for Cell);
