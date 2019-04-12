extern crate num;

use std::ops::{Shl, ShlAssign, Shr, ShrAssign, Rem, RemAssign, BitOrAssign, BitXor, Not, Sub, BitAnd, BitOr};
use std::fmt::Debug;
use std::marker::PhantomData;

use num::Num;
use num::FromPrimitive;
use num::clamp;


trait Lens<A> {
    fn get(&self) -> A;

    fn set(&mut self, a: A);

    fn modify<F: Fn(A) -> A>(&mut self, f: F) {
        self.set(f(self.get()));
    }
}

trait Scope<A, I>: Lens<A> {
    fn adjust(&mut self, index: I);
}


/* BitWise trait for Primitive Types */
trait BitWise: Sized + Copy + Debug + PartialOrd + Shl<u32, Output = Self> +
               ShlAssign<u32> + Shr<u32, Output = Self> + ShrAssign<u32> +
               Rem<Self, Output = Self> + RemAssign<Self> + BitOrAssign<Self>
               + BitXor<Self, Output = Self> + Not<Output = Self> +
               Sub<Self, Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> + Num +
               FromPrimitive

{}

impl BitWise for u8  {}
impl BitWise for u16 {}
impl BitWise for u32 {}
impl BitWise for u64 {}


/* Vec Scope */
#[derive(Clone, PartialEq, Eq)]
struct VecScope<A> {
    vec: Vec<A>,
    pos: usize,
}

// NOTE this does not account for empty vectors
// Lens should probably return Option<A>
impl<A> VecScope<A> {
    fn with_vec(vec: Vec<A>) -> VecScope<A> {
        VecScope {
            vec: vec,
            pos: 0,
        }
    }
}

impl<A: Copy> Lens<A> for VecScope<A> {
    fn get(&self) -> A {
        self.vec[self.pos]
    }

    fn set(&mut self, a: A) {
        self.vec[self.pos] = a;
    }
}

impl <A: Copy> Scope<A, usize> for VecScope<A> {
    fn adjust(&mut self, pos: usize) {
        self.pos = clamp(pos, 0, self.vec.len() - 1);
    }
}

impl <A: Copy> Scope<A, isize> for VecScope<A> {
    fn adjust(&mut self, offset: isize) {
        self.pos = clamp((self.pos as isize) + offset, 0, (self.vec.len() - 1) as isize) as usize;
    }
}

#[test]
fn test_vec_scope() {
    let mut vec_scope = VecScope::with_vec(vec![1,2,3,4,5]);

    assert_eq!(vec_scope.get(), 1);

    vec_scope.set(100);
    assert_eq!(vec_scope.get(), 100);

    vec_scope.adjust(1isize);
    assert_eq!(vec_scope.get(), 2);

    vec_scope.adjust(1isize);
    assert_eq!(vec_scope.get(), 3);

    vec_scope.adjust(3usize);
    assert_eq!(vec_scope.get(), 4);

    vec_scope.adjust(100usize);
    assert_eq!(vec_scope.get(), 5);

    vec_scope.adjust(-1isize);
    assert_eq!(vec_scope.get(), 4);

    vec_scope.adjust(100isize);
    assert_eq!(vec_scope.get(), 5);

    vec_scope.set(500);
    assert_eq!(vec_scope.get(), 500);

    vec_scope.adjust(-100isize);
    assert_eq!(vec_scope.get(), 100);
}


/* Bit Vec Scope */
#[derive(Clone, PartialEq, Eq)]
struct BitVecScope {
    bytes: Vec<u8>,
    pos: usize,
}

impl BitVecScope {
    fn with_bytes(bytes: Vec<u8>) -> BitVecScope {
        BitVecScope {
            bytes: bytes,
            pos: 0,
        }
    }
}

impl Lens<bool> for BitVecScope {
    fn get(&self) -> bool {
        let index = self.pos / 8;
        let bit_index = self.pos % 8;
        (self.bytes[index] & (1 << bit_index)) != 0
    }

    fn set(&mut self, a: bool) {
        let index = self.pos / 8;
        let bit_index = self.pos % 8;
        self.bytes[index] = (self.bytes[index] & !(1 << bit_index)) | ((a as u8) << bit_index);
    }
}

impl Scope<bool, usize> for BitVecScope {
    fn adjust(&mut self, pos: usize) {
        self.pos = clamp(pos, 0, (self.bytes.len() * 8) - 1);
    }
}

impl Scope<bool, isize> for BitVecScope {
    fn adjust(&mut self, offset: isize) {
        self.pos = clamp((self.pos as isize) + offset, 0, ((8 * self.bytes.len()) - 1) as isize) as usize;
    }
}

#[test]
fn test_bit_vec_scope() {
    let mut bit_vec_scope = BitVecScope::with_bytes(vec![1,2,3,4,0x80]);

    assert_eq!(bit_vec_scope.get(), true);

    bit_vec_scope.set(false);
    assert_eq!(bit_vec_scope.get(), false);

    bit_vec_scope.adjust(1usize);
    bit_vec_scope.set(true);
    assert_eq!(bit_vec_scope.bytes[0], 0x02);

    bit_vec_scope.adjust(100isize);
    assert_eq!(bit_vec_scope.get(), true);
}


/* Bit Word Scope */
struct BitWordScope<B> {
    vec: Vec<B>,
    bits_used: usize,
    pos: usize,
}

impl<B> BitWordScope<B> {
    fn with_words(vec: Vec<B>, bits_used: usize) -> BitWordScope<B> {
        BitWordScope {
            vec: vec,
            bits_used: bits_used,
            pos: 0,
        }
    }
}

impl<B: BitWise> Lens<bool> for BitWordScope<B> {
    fn get(&self) -> bool {
        let index = self.pos / self.bits_used;
        let bit_index = (self.pos % self.bits_used) as u32;
        (self.vec[index] & (B::one() << bit_index)) != B::zero()
    }

    fn set(&mut self, a: bool) {
        let index = self.pos / self.bits_used;
        let bit_index = (self.pos % self.bits_used) as u32;

        let loc_cleared = self.vec[index] & !(B::one() << bit_index);
        let set_bit = B::from_u8(a as u8).unwrap() << bit_index;
        let loc_set = loc_cleared | set_bit;
        self.vec[index] = loc_set;
    }
}

impl<B: BitWise> Lens<B> for BitWordScope<B> {
    fn get(&self) -> B {
        let index = self.pos / self.bits_used;
        self.vec[index]
    }

    fn set(&mut self, a: B) {
        let index = self.pos / self.bits_used;
        self.vec[index] = a;
    }
}

impl<B: BitWise> Scope<B, usize> for BitWordScope<B> {
    fn adjust(&mut self, pos: usize) {
        self.pos = clamp(pos, 0, (self.vec.len() * self.bits_used) - 1);
    }
}

impl<B: BitWise> Scope<B, isize> for BitWordScope<B> {
    fn adjust(&mut self, offset: isize) {
        self.pos = clamp((self.pos as isize) + offset, 0, ((self.bits_used * self.vec.len()) - 1) as isize) as usize;
    }
}

#[test]
fn test_bit_word_scope() {
    let mut bit_word_scope: BitWordScope<u8> = BitWordScope::with_words(vec![1,2,3,4,0x7], 3);

    let current: bool = bit_word_scope.get();
    assert_eq!(current, true);

    let current: u8 = bit_word_scope.get();
    assert_eq!(current, 1);

    bit_word_scope.set(false);
    let current: bool = bit_word_scope.get();
    assert_eq!(current, false);

    bit_word_scope.adjust(1usize);
    bit_word_scope.set(true);
    assert_eq!(bit_word_scope.vec[0], 0x02);

    bit_word_scope.adjust(100isize);
    let current: bool = bit_word_scope.get();
    assert_eq!(current, true);

    let current: u8 = bit_word_scope.get();
    assert_eq!(current, 0x07);
}


/* Packed Bit Scope */
#[derive(Clone, PartialEq, Eq)]
struct PackedBitScope<B> {
    bytes: Vec<u8>,
    pos: usize,
    bits_used: u8,
    marker: PhantomData<B>
}

impl<B> PackedBitScope<B> {
    fn with_words(bytes: Vec<u8>, bits_used: u8) -> PackedBitScope<B> {
        PackedBitScope {
            bytes: bytes,
            pos: 0,
            bits_used: bits_used,
        }
    }
}

impl<B> Lens<B> for PackedBitScope<B> {
    fn get(&self) -> B {
        let index = self.pos / 8;
        let bit_index = self.pos % 8;
        (self.bytes[index] & (1 << bit_index)) != 0
    }

    fn set(&mut self, a: B) {
        let index = self.pos / 8;
        let bit_index = self.pos % 8;
        self.bytes[index] = (self.bytes[index] & !(1 << bit_index)) | ((a as u8) << bit_index);
    }
}

impl<B> Scope<B, usize> for PackedBitScope<B> {
    fn adjust(&mut self, pos: usize) {
        // NOTE does not take into account extra bits at end of last byte
        self.pos = clamp(pos, 0, (self.bytes.len() * 8) - 1);
    }
}

impl<B> Scope<B, isize> for PackedBitScope<B> {
    fn adjust(&mut self, offset: isize) {
        // NOTE does not take into account extra bits at end of last byte
        self.pos = clamp((self.pos as isize) + offset, 0, ((8 * self.bytes.len()) - 1) as isize) as usize;
    }
}

#[test]
fn test_bit_vec_scope() {
    let mut bit_vec_scope = PackedBitScope::with_bytes(vec![1,2,3,4,0x80]);

    assert_eq!(bit_vec_scope.get(), true);

    bit_vec_scope.set(false);
    assert_eq!(bit_vec_scope.get(), false);

    bit_vec_scope.adjust(1usize);
    bit_vec_scope.set(true);
    assert_eq!(bit_vec_scope.bytes[0], 0x02);

    bit_vec_scope.adjust(100isize);
    assert_eq!(bit_vec_scope.get(), true);
}
