#[macro_use]extern crate criterion;
extern crate rand;

extern crate scope;

use core::ops::Range;

use rand::Rng;
use criterion::Criterion;
use myopic::lens::*;
use scope::*;

const LENGTH: usize = 1_000_000;
//const LENGTH: usize = 100;

const STEP: usize = 1000;

fn packed_bits_scope(c: &mut Criterion) { let mut packed_scope = PackedBitScope::with_words(vec!(0; LENGTH), 8);
    let packed_lens = PackedBitScope::num_lens::<u8>();

    c.bench_function("packed_words_8", move |b| b.iter(|| {
        packed_scope.adjust(0usize);
        for _ in 0..packed_scope.shape() {
            (packed_lens.set)(&mut packed_scope, 1);
            packed_scope.adjust(1isize);
        }
    }));

    let mut num_bytes_needed = LENGTH / 8;
    if LENGTH % 8 != 0 {
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
    let mut num_bytes_needed = LENGTH / 8;
    if LENGTH % 8 != 0 {
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

fn vec_raw(c: &mut Criterion) {
    let mut vec = vec!(0; LENGTH);

    c.bench_function("vec_raw", move |b| b.iter(|| {
        for i in 0..vec.len() {
            vec[i] = 1;
        }
    }));
}

fn vec_scope(c: &mut Criterion) {
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();
    let vec_lens = VecScope::lens();

    c.bench_function("vec_8", move |b| b.iter(|| {
        for _ in 0..vec_scope.shape() {
            vec_lens.set(&mut vec_scope, 1);
            vec_scope.adjust(1isize);
        }
    }));
}

fn vec_raw_step(c: &mut Criterion) {
    let mut vec = vec!(0; LENGTH);

    c.bench_function("vec_raw_step", move |b| b.iter(|| {
        for i in (0..vec.len()).step_by(STEP) {
            vec[i] = 1;
        }
    }));
}

fn vec_scope_step(c: &mut Criterion) {
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();
    let vec_lens = VecScope::lens();

    c.bench_function("vec_8_step", move |b| b.iter(|| {
        for i in (0..vec_scope.shape()).step_by(STEP) {
            vec_lens.set(&mut vec_scope, 1);
            vec_scope.adjust(i);
        }
    }));
}

fn bit_vec_scope(c: &mut Criterion) {
    let mut num_bytes_needed = LENGTH / 8;
    if LENGTH % 8 != 0 {
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
    let mut indices = Vec::with_capacity(LENGTH);
    let mut rng = rand::thread_rng();
    for _ in 0..LENGTH {
        indices.push(rng.gen_range(0usize, LENGTH));
    }

    let mut packed_scope = PackedBitScope::with_words(vec!(0; LENGTH), 8);
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
    let mut indices = Vec::with_capacity(LENGTH);
    let mut rng = rand::thread_rng();
    for _ in 0..LENGTH {
        indices.push(rng.gen_range(0usize, LENGTH));
    }
    let mut num_bytes_needed = LENGTH / 8;
    if LENGTH % 8 != 0 {
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
    let mut indices = Vec::with_capacity(LENGTH);
    let mut rng = rand::thread_rng();
    for _ in 0..LENGTH {
        indices.push(rng.gen_range(0usize, LENGTH));
    }
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();
    let vec_lens = VecScope::lens();
    c.bench_function("vec_8_random", move |b| b.iter(|| {
        for index in indices.iter() {
            vec_lens.set(&mut vec_scope, 1);
            vec_scope.adjust(*index);
        }
    }));
}

fn map_seq(c: &mut Criterion) {
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..LENGTH).step_by(STEP),
                                  |val| val + 1);

    let transform2 =
        Transform::make_transform(VecScope::lens(),
                                  (0..LENGTH).step_by(STEP),
                                  |val| val + 1);

    c.bench_function("map_seq", move |b| b.iter(|| {
        transform1.transform(&mut vec_scope);
        transform2.transform(&mut vec_scope);
    }));
}

fn map_seq_4(c: &mut Criterion) {
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..LENGTH).step_by(STEP),
                                  |val| val + 1);

    let transform2 =
        Transform::make_transform(VecScope::lens(),
                                  (0..LENGTH).step_by(STEP),
                                  |val| val + 1);

    let transform3 =
        Transform::make_transform(VecScope::lens(),
                                  (0..LENGTH).step_by(STEP),
                                  |val| val + 1);

    let transform4 =
        Transform::make_transform(VecScope::lens(),
                                  (0..LENGTH).step_by(STEP),
                                  |val| val + 1);

    c.bench_function("map_seq_4", move |b| b.iter(|| {
        transform1.transform(&mut vec_scope);
        transform2.transform(&mut vec_scope);
        transform3.transform(&mut vec_scope);
        transform4.transform(&mut vec_scope);
    }));
}

fn map_single(c: &mut Criterion) {
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();
    let transform =
        Transform::make_transform(VecScope::lens(),
                                  (0..LENGTH).step_by(STEP),
                                  |val| val + 1);

    c.bench_function("map_single", move |b| b.iter(|| {
        transform.transform(&mut vec_scope);
    }));
}

fn map_both(c: &mut Criterion) {
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..LENGTH).step_by(STEP),
                                  |val| val + 1);

    let transform2 =
        Transform::make_transform(VecScope::lens(),
                                  (0..LENGTH).step_by(STEP),
                                  |val| val + 1);

    c.bench_function("map_both", move |b| b.iter(|| {
        apply_both(&transform1, &transform2, &mut vec_scope);
    }));
}

fn map_many_2(c: &mut Criterion) {
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();

    c.bench_function("map_many_2", move |b| b.iter(|| {
        let transform1 =
            Transform::make_transform(VecScope::lens(),
                                      (0..LENGTH).step_by(STEP),
                                      |val| val + 1);

        let transform2 =
            Transform::make_transform(VecScope::lens(),
                                      (0..LENGTH).step_by(STEP),
                                      |val| val + 1);
        let transforms = vec!(transform1, transform2);
        apply_many(transforms, &mut vec_scope);
    }));
}

fn map_many_3(c: &mut Criterion) {
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();

    c.bench_function("map_many_3", move |b| b.iter(|| {
        let transform1 =
            Transform::make_transform(VecScope::lens(),
                                      (0..LENGTH).step_by(STEP),
                                      |val| val + 1);

        let transform2 =
            Transform::make_transform(VecScope::lens(),
                                      (0..LENGTH).step_by(STEP),
                                      |val| val + 1);

        let transform3 =
            Transform::make_transform(VecScope::lens(),
                                      (0..LENGTH).step_by(STEP),
                                      |val| val + 1);
        let transforms = vec!(transform1, transform2, transform3);
        apply_many(transforms, &mut vec_scope);
    }));
}

fn map_many_4(c: &mut Criterion) {
    let mut vec_scope = VecScope::with_vec(vec!(0; LENGTH)).unwrap();

    c.bench_function("map_many_4", move |b| b.iter(|| {
        let transform1 =
            Transform::make_transform(VecScope::lens(),
                                      (0..LENGTH).step_by(STEP),
                                      |val| val + 1);

        let transform2 =
            Transform::make_transform(VecScope::lens(),
                                      (0..LENGTH).step_by(STEP),
                                      |val| val + 1);

        let transform3 =
            Transform::make_transform(VecScope::lens(),
                                      (0..LENGTH).step_by(STEP),
                                      |val| val + 1);

        let transform4 =
            Transform::make_transform(VecScope::lens(),
                                      (0..LENGTH).step_by(STEP),
                                      |val| val + 1);

        let transforms = vec!(transform1, transform2, transform3, transform4);
        apply_many(transforms, &mut vec_scope);
    }));
}

criterion_group!(packing, packed_bits_scope, packed_bits_scope_bool, vec_scope, bit_vec_scope, vec_raw, vec_raw_step, vec_scope_step);
criterion_group!(random, packed_bit_8_random_access, packed_bit_1_random_access, vec_random_access);
criterion_group!(mapping, map_single, map_seq, map_seq_4, map_both, map_many_2, map_many_3, map_many_4);

criterion_main!(packing, random, mapping);

