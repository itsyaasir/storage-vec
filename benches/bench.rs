#![feature(test)]

extern crate test;

use storage_vec::Storage;
use test::bench::{benchmark, black_box};

#[bench]
fn bench_add(b: &mut test::Bencher) {
    let mut storage = Storage::with_capacity(20);
    b.iter(|| {
        storage.add(black_box(1));
    });
}

//YASIR
