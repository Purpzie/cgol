//! Contains the [`Game`] struct and its iterators.

mod iter;
mod panic;

use crate::cell::Cell;
use alloc::{vec, vec::Vec};
use core::ops::{Index, IndexMut};
pub use iter::*;
#[cfg(any(test, feature = "use-rand"))]
use rand::distributions::{Bernoulli, Distribution};

/// An instance of Conway's Game of Life.
///
/// TODO: docs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    cells: Vec<Cell>,
    next: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Game {
    /// Creates a new instance of Conway's Game of Life.
    ///
    /// # Panics
    /// Panics if `width < 1`, `height < 1`, or if `width * height > isize::MAX`.
    pub fn new(width: usize, height: usize) -> Game {
        if width == 0 {
            panic::width_is_zero();
        } else if height == 0 {
            panic::height_is_zero();
        }

        // doesn't need documentation since it's already > isize::MAX
        let area = width.checked_mul(height).expect("width * height overflow");
        let cells = vec![Cell::Dead; area];
        let next = cells.clone();

        Self {
            width,
            height,
            cells,
            next,
        }
    }

    /// Ticks once.
    ///
    /// # Examples
    /// ```
    /// # use cgol::Game;
    /// let mut game = Game::new(100, 100);
    /// game.fill_random(0.5);
    /// game.tick();
    /// // view the game somehow...
    /// ```
    pub fn tick(&mut self) {
        /*
        Safety requires the following to be true:
          1. cells.len() == next.len() == width * height
          2. width != 0 && height != 0

        Even though we already try to ensure that these are always true, we check just in case. This
        way, we don't need to check if we're in bounds during the main loop. In practice, panics
        like this should be caught in testing and should never appear in release, but the check is
        left in release to ensure with 101% confidence that we'll never cause UB.

        It's not really that expensive to do anyways.
        */
        let area = self.width * self.height;
        if self.cells.len() != area || self.next.len() != area {
            panic::fatal_len(self);
        } else if self.width == 0 || self.height == 0 {
            panic::fatal_width_height(self);
        }

        // cache
        let row_max = self.height - 1;
        let col_max = self.width - 1;

        // needed later, see the end of the col loop
        let mut current_index = 0;

        for mut row in 0..self.height {
            // wrap up & down to be within row_max (this is faster than modulo)
            let mut up = if row == 0 { row_max } else { row - 1 };
            let mut down = if row == row_max { 0 } else { row + 1 };

            // cells are stored in a flat vec, so we need to get the correct indexes.
            // each one refers to the first cell in their respective row
            up *= self.width;
            row *= self.width;
            down *= self.width;

            // also don't let them be changed later
            let (up, row, down) = (up, row, down);

            for col in 0..self.width {
                // wrap left & right to be within col_max
                let left = if col == 0 { col_max } else { col - 1 };
                let right = if col == col_max { 0 } else { col + 1 };

                /*
                SAFETY:
                  1. up, row, and down are indexes to the first cell in a row
                  2. therefore, there are always col_max cells to the right
                  3. left and right <= col_max since they're wrapped to be within it
                  4. col <= col_max due to the range it iterates over
                */
                unsafe {
                    let neighbors: [usize; 8] = [
                        up + left,    // top left
                        up + col,     // top
                        up + right,   // top right
                        row + right,  // right
                        down + right, // bottom right
                        down + col,   // bottom
                        down + left,  // bottom left
                        row + left,   // left
                    ];

                    // TODO(rust 1.51): use core::array::IntoIter
                    let neighbor_count = neighbors
                        .iter()
                        .fold(0u8, |i, &c| i + *self.cells.get_unchecked(c) as u8);

                    /*
                    SAFETY:
                      1. the col loop runs exactly width * height times
                      2. width * height == cells.len() == next.len()
                        a. ^ see the beginning of this function
                      3. current_index starts at 0, then incremenets after the col loop
                      4. therefore it is always in bounds before the increment

                    `current_index = row + col` also works, but it's 10% slower for some reason :/
                    so now we need to declare a variable aaaaall the way at the start of the loop
                    */
                    *self.next.get_unchecked_mut(current_index) = match neighbor_count {
                        3 => Cell::Alive,
                        2 => *self.cells.get_unchecked(current_index),
                        _ => Cell::Dead,
                    };
                } // end unsafe block

                current_index += 1;
            } // end col loop
        } // end row loop

        self.cells.copy_from_slice(&self.next);
    } // end tick()

    /// Gets this game's width.
    ///
    /// # Examples
    /// ```
    /// # use cgol::Game;
    /// let game = Game::new(69, 420);
    /// assert_eq!(game.width(), 69);
    /// ```
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets this game's height.
    ///
    /// # Examples
    /// ```
    /// # use cgol::Game;
    /// let game = Game::new(69, 420);
    /// assert_eq!(game.height(), 420);
    /// ```
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Gets this game's area.
    ///
    /// # Examples
    /// ```
    /// # use cgol::Game;
    /// let game = Game::new(4, 3);
    /// assert_eq!(game.area(), 12);
    /// ```
    #[inline]
    pub fn area(&self) -> usize {
        #[cfg(debug_assertions)]
        assert!(self.cells.len() == self.width * self.height);

        self.cells.len()
    }

    /// Gets a specific cell, returning `None` if out of bounds.
    ///
    /// # Example
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let game = Game::new(10, 10);
    ///
    /// assert_eq!(game.get(2, 3), Some(Dead));
    /// assert_eq!(game.get(69, 420), None);
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        if row < self.height && col < self.width {
            Some(self.cells[row * self.width + col])
        } else {
            None
        }
    }

    /// Mutably gets a specific cell, returning `None` if out of bounds.
    ///
    /// # Example
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let mut game = Game::new(3, 3);
    ///
    /// if let Some(cell) = game.get_mut(0, 0) {
    ///     *cell = Alive;
    /// }
    ///
    /// assert_eq!(game.get_row(0), &[Alive, Dead, Dead]);
    /// ```
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        if row < self.height && col < self.width {
            Some(&mut self.cells[row * self.width + col])
        } else {
            None
        }
    }

    /// Gets a slice of a specific row.
    ///
    /// Due to how cells are stored, this isn't possible for columns.
    ///
    /// # Panics
    /// Panics if `row` is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let mut game = Game::new(3, 5);
    /// assert_eq!(game.get_row(0), &[Dead, Dead, Dead]);
    /// ```
    pub fn get_row(&self, row: usize) -> &[Cell] {
        if row >= self.height {
            panic::height(row, self.height);
        }

        let begin = row * self.width;
        let end = begin + self.width;

        &self.cells[begin..end]
    }

    /// Gets a mutable slice of a specific row.
    ///
    /// Due to how cells are stored, this isn't possible for columns.
    ///
    /// # Panics
    /// Panics if `row` is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let mut game = Game::new(3, 5);
    /// let mut row = game.get_row_mut(0);
    /// row[0] = Alive;
    ///
    /// assert_eq!(game.get_row(0), &[Alive, Dead, Dead]);
    /// ```
    pub fn get_row_mut(&mut self, row: usize) -> &mut [Cell] {
        if row >= self.height {
            panic::height(row, self.height);
        }

        let begin = row * self.width;
        let end = begin + self.width;

        &mut self.cells[begin..end]
    }

    /// Gets an iterator over the cells in a specific row.
    ///
    /// # Panics
    /// Panics if `row` is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let game = Game::new(3, 5);
    /// let mut row = game.row(0);
    ///
    /// assert_eq!(row.next(), Some(Dead));
    /// assert_eq!(row.next(), Some(Dead));
    /// assert_eq!(row.next(), Some(Dead));
    /// assert_eq!(row.next(), None);
    /// ```
    pub fn row(&self, row: usize) -> Row {
        self.get_row(row).iter().copied()
    }

    /// Gets a mutable iterator over the cells in a specific row.
    ///
    /// # Panics
    /// Panics if `row` is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let mut game = Game::new(3, 5);
    ///
    /// for cell in game.row_mut(0) {
    ///     *cell = Alive;
    /// }
    ///
    /// assert_eq!(game.get_row(0), &[Alive, Alive, Alive]);
    /// # assert_eq!(game.get_row(1), &[Dead, Dead, Dead]);
    /// ```
    pub fn row_mut(&mut self, row: usize) -> RowMut {
        self.get_row_mut(row).iter_mut()
    }

    /// Gets an iterator over the cells in a specific column.
    ///
    /// # Panics
    /// Panics if `col` is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let game = Game::new(7, 3);
    /// let mut col = game.col(0);
    ///
    /// assert_eq!(col.next(), Some(Dead));
    /// assert_eq!(col.next(), Some(Dead));
    /// assert_eq!(col.next(), Some(Dead));
    /// assert_eq!(col.next(), None);
    /// ```
    pub fn col(&self, col: usize) -> Col {
        if col >= self.width {
            panic::width(col, self.width);
        }

        self.cells.iter().copied().skip(col).step_by(self.width)
    }

    /// Gets a mutable iterator over the cells in a specific column.
    ///
    /// # Panics
    /// Panics if `col` is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let mut game = Game::new(7, 3);
    ///
    /// for cell in game.col_mut(0) {
    ///     *cell = Alive;
    /// }
    ///
    /// let mut col: Vec<_> = game.col(0).collect();
    /// assert_eq!(&col, &[Alive, Alive, Alive]);
    /// # let mut col_2_electric_boogaloo: Vec<_> = game.col(1).collect();
    /// # assert_eq!(&col_2_electric_boogaloo, &[Dead, Dead, Dead]);
    /// ```
    pub fn col_mut(&mut self, col: usize) -> ColMut {
        if col >= self.width {
            panic::width(col, self.width);
        }

        self.cells.iter_mut().skip(col).step_by(self.width)
    }

    /// Kills all cells.
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let mut game = Game::new(10, 10);
    /// game[(0, 0)] = Alive;
    /// game.clear();
    /// assert_eq!(game[(0, 0)], Dead);
    /// ```
    pub fn clear(&mut self) {
        self.cells.fill(Cell::Dead);
    }

    /// Returns `true` if all cells are [`Dead`](Cell::Dead).
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let mut game = Game::new(10, 10);
    /// assert!(game.all_dead());
    ///
    /// game[(0, 0)] = Alive;
    /// assert!(!game.all_dead())
    /// ```
    pub fn all_dead(&self) -> bool {
        fn is_dead(&c: &Cell) -> bool {
            c == Cell::Dead
        }

        self.cells.iter().all(is_dead)
    }

    /// Returns `true` if all cells are [`Alive`](Cell::Alive).
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let mut game = Game::new(10, 10);
    /// assert!(!game.all_alive());
    ///
    /// game.invert();
    /// assert!(game.all_alive());
    /// ```
    pub fn all_alive(&self) -> bool {
        fn is_alive(&c: &Cell) -> bool {
            c == Cell::Alive
        }

        self.cells.iter().all(is_alive)
    }

    /// Inverts all cells.
    ///
    /// # Examples
    /// ```
    /// # use cgol::{Game, Cell::*};
    /// let mut game = Game::new(10, 10);
    /// game.invert();
    /// assert!(game.all_alive());
    /// ```
    pub fn invert(&mut self) {
        for cell in &mut self.cells {
            *cell = !*cell;
        }
    }

    /// Fills the game's cells randomly with a probability of being alive.
    ///
    /// # Panics
    /// Panics if `chance` is not in the range `[0, 1]`.
    ///
    /// # Examples
    /// ```
    /// # use cgol::Game;
    /// let mut game = Game::new(10, 10);
    ///
    /// game.fill_random(1.0);
    /// assert!(game.all_alive());
    ///
    /// game.fill_random(0.0);
    /// assert!(game.all_dead());
    ///
    /// game.fill_random(0.5);
    /// assert!(!game.all_dead());
    /// ```
    #[cfg(any(test, feature = "use-rand"))]
    pub fn fill_random(&mut self, chance: f64) {
        let mut rng = Bernoulli::new(chance)
            .unwrap()
            .sample_iter(rand::thread_rng());

        for cell in &mut self.cells {
            *cell = match rng.next() {
                Some(val) => Cell::from(val as u8),
                None => unreachable!(),
            };
        }
    }
}

impl Index<(usize, usize)> for Game {
    type Output = Cell;

    fn index(&self, (col, row): (usize, usize)) -> &Cell {
        if row >= self.height {
            panic::height(row, self.height);
        } else if col >= self.width {
            panic::width(col, self.width);
        }

        &self.cells[row * self.width + col]
    }
}

impl IndexMut<(usize, usize)> for Game {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Cell {
        if row >= self.height {
            panic::height(row, self.height);
        } else if col >= self.width {
            panic::width(col, self.width);
        }

        &mut self.cells[row * self.width + col]
    }
}

// https://rust-lang.github.io/api-guidelines/interoperability#c-conv-traits
impl AsRef<Game> for Game {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsMut<Game> for Game {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
