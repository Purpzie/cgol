//! Contains [`Game`]'s iterators.

use crate::cell::Cell;
use core::{
    iter::{Copied, Skip, StepBy},
    slice::{Iter, IterMut},
};

#[cfg(doc)]
use crate::game::Game;

// TODO: make these newtypes

/// An iterator over the cells in a specific row.
///
/// This is created by the [`Game::row`] method.
pub type Row<'a> = Copied<Iter<'a, Cell>>;

/// A mutable iterator over the cells in a specific row.
///
/// This is created by the [`Game::row_mut`] method.
pub type RowMut<'a> = IterMut<'a, Cell>;

/// An iterator over the cells in a specific column.
///
/// This is created by the [`Game::col`] method.
pub type Col<'a> = StepBy<Skip<Copied<Iter<'a, Cell>>>>;

/// A mutable iterator over the cells in a specific column.
///
/// This is created by the [`Game::col_mut`] method.
pub type ColMut<'a> = StepBy<Skip<IterMut<'a, Cell>>>;
