//! Contains panics used by [`Game`]. They are here to reduce code size as much as possible.

use super::Game;

#[cold]
#[inline(never)]
#[track_caller]
pub(super) fn width_is_zero() {
    panic!("width must not be 0");
}

#[cold]
#[inline(never)]
#[track_caller]
pub(super) fn height_is_zero() {
    panic!("height must not be 0");
}

#[cold]
#[inline(never)]
#[track_caller]
pub(super) fn width(val: usize, width: usize) {
    panic!(
        "index out of bounds: {} is {} but the index is {}",
        "width", width, val
    );
}

#[cold]
#[inline(never)]
#[track_caller]
pub(super) fn height(val: usize, height: usize) {
    panic!(
        "index out of bounds: {} is {} but the index is {}",
        "height", height, val
    );
}

#[cold]
#[inline(never)]
#[track_caller]
pub(super) fn fatal_len(g: &Game) {
    panic!(
        "fatal error in cgol crate: len is not {} ({} * {})\ncells.len(): {}\nnext.len(): {}\n\
        This is a bug. Please file an issue at https://github.com/Purpzie/cgol/issues/",
        g.width * g.height,
        g.width,
        g.height,
        g.cells.len(),
        g.next.len(),
    );
}

#[cold]
#[inline(never)]
#[track_caller]
pub(super) fn fatal_width_height(g: &Game) {
    panic!(
        "fatal error in cgol crate: width or height are 0\nwidth: {}\nheight: {}\n\
        This is a bug. Please file an issue at https://github.com/Purpzie/cgol/issues/",
        g.width, g.height,
    );
}
