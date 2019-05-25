#[macro_use]extern crate criterion;

extern crate scope;


use criterion::Criterion;
use scope::*;


fn packed_bits_scope(c: &mut Criterion) {
    let length = 10000;

    let mut packed_scope = PackedBitScope::with_words(vec!(0; length), 8);
    let packed_lens = PackedBitScope::num_lens::<u8>();

    c.bench_function("packed_words_8", move |b| b.iter(|| {
        packed_scope.adjust(0usize);
        for _ in 0..packed_scope.shape() {
            (packed_lens.set)(&mut packed_scope, 1);
            packed_scope.adjust(1isize);
        }
    }));

    let mut num_bytes_needed = length / 8;
    if length % 8 != 0 {
        num_bytes_needed += 1;
    }
    let mut packed_scope = PackedBitScope::with_words(vec!(0; num_bytes_needed), 1);
    let packed_lens = PackedBitScope::num_lens::<u8>();

    c.bench_function("packed_words_1", move |b| b.iter(|| {
        packed_scope.adjust(0usize);
        for _ in 0..packed_scope.shape() {
            (packed_lens.set)(&mut packed_scope, 1);
            packed_scope.adjust(1isize);
        }
    }));
}

fn packed_bits_scope_bool(c: &mut Criterion) {
    let length = 10000;

    let mut num_bytes_needed = length / 8;
    if length % 8 != 0 {
        num_bytes_needed += 1;
    }
    let mut packed_scope = PackedBitScope::with_words(vec!(0; num_bytes_needed), 1);
    let packed_lens = PackedBitScope::bit_lens();

    c.bench_function("packed_bits", move |b| b.iter(|| {
        packed_scope.adjust(0usize);
        for _ in 0..packed_scope.shape() {
            (packed_lens.set)(&mut packed_scope, true);
            packed_scope.adjust(1isize);
        }
    }));
}

fn vec_scope(c: &mut Criterion) {
    let length = 10000;

    let mut vec_scope = VecScope::with_vec(vec!(0; length)).unwrap();
    let vec_lens = VecScope::lens();

    c.bench_function("vec_8", move |b| b.iter(|| {
        for _ in 0..vec_scope.shape() {
            (vec_lens.set)(&mut vec_scope, 1);
            vec_scope.adjust(1isize);
        }
    }));
}

criterion_group!(packing, packed_bits_scope, packed_bits_scope_bool, vec_scope);
criterion_main!(packing);

