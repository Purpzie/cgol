#![feature(test)]

extern crate test;
use cgol::*;
use test::Bencher;

#[bench]
#[cfg(feature = "use-rand")]
fn tick(b: &mut Bencher) {
    let mut game = Game::new(1000, 1000); // 1 million cells
    game.fill_random(0.5);
    b.iter(move || game.tick());
}

#[bench]
#[cfg(not(feature = "use-rand"))]
fn use_all_features(_: &mut Bencher) {
    panic!("You forgot to use --all-features!");
}
