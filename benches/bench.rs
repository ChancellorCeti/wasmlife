#![feature(test)]

extern crate test;
extern crate wasmlife;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = wasmlife::Universe::new(64,64);

    b.iter(|| {
        universe.tick();
    });
}