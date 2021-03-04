//! [![Tests](https://img.shields.io/github/workflow/status/Purpzie/cgol/CI)](https://github.com/Purpzie/cgol/actions/workflows/ci.yml)
//!
//! This crate is a work in progress.

#![no_std]
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png")]

extern crate alloc;

mod cell;
pub mod game;

pub use cell::Cell;
#[doc(inline)]
pub use game::Game;
pub use Cell::{Alive, Dead};
