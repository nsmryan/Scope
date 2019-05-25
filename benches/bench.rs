#[macro_use]extern crate criterion;
extern crate rand;

extern crate scope;


use rand::Rng;
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

fn bit_vec_scope(c: &mut Criterion) {
    let length = 10000;

    let mut num_bytes_needed = length / 8;
    if length % 8 != 0 {
        num_bytes_needed += 1;
    }

    let mut bit_vec_scope = BitVecScope::with_bytes(vec!(0; num_bytes_needed)).unwrap();
    let bit_vec_lens = BitVecScope::lens();

    c.bench_function("bit_vec_8_random", move |b| b.iter(|| {
        for _ in 0..bit_vec_scope.shape() {
            (bit_vec_lens.set)(&mut bit_vec_scope, true);
            bit_vec_scope.adjust(1isize);
        }
    }));
}

fn packed_bit_8_random_access(c: &mut Criterion) {
    let length: usize = 10000;

    let mut indices = Vec::with_capacity(length);
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        indices.push(rng.gen_range(0usize, length));
    }

    let mut packed_scope = PackedBitScope::with_words(vec!(0; length), 8);
    let packed_lens = PackedBitScope::num_lens::<u8>();
    c.bench_function("packed_words_8_random", move |b| b.iter(|| {
        packed_scope.adjust(0usize);
        for index in indices.iter() {
            (packed_lens.set)(&mut packed_scope, 1);
            packed_scope.adjust(*index);
        }
    }));
}

fn packed_bit_1_random_access(c: &mut Criterion) {
    let length: usize = 10000;

    let mut indices = Vec::with_capacity(length);
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        indices.push(rng.gen_range(0usize, length));
    }
    let mut num_bytes_needed = length / 8;
    if length % 8 != 0 {
        num_bytes_needed += 1;
    }
    let mut packed_scope = PackedBitScope::with_words(vec!(0; num_bytes_needed), 1);
    let packed_lens = PackedBitScope::num_lens::<u8>();
    c.bench_function("packed_words_1_random", move |b| b.iter(|| {
        packed_scope.adjust(0usize);
        for index in indices.iter() {
            (packed_lens.set)(&mut packed_scope, 1);
            packed_scope.adjust(*index);
        }
    }));
}

fn vec_random_access(c: &mut Criterion) {
    let length: usize = 10000;

    let mut indices = Vec::with_capacity(length);
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        indices.push(rng.gen_range(0usize, length));
    }
    let mut vec_scope = VecScope::with_vec(vec!(0; length)).unwrap();
    let vec_lens = VecScope::lens();
    c.bench_function("vec_8_random", move |b| b.iter(|| {
        for index in indices.iter() {
            (vec_lens.set)(&mut vec_scope, 1);
            vec_scope.adjust(*index);
        }
    }));
}

criterion_group!(packing, packed_bits_scope, packed_bits_scope_bool, vec_scope, bit_vec_scope);
criterion_group!(random, packed_bit_8_random_access, packed_bit_1_random_access, vec_random_access);
criterion_main!(packing, random);

