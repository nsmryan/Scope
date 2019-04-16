extern crate num;

mod lens;
mod scope;
mod vec_scope;
mod bit_vec_scope;
mod bit_word_scope;
mod shape;


use std::ops::{Shl, ShlAssign, Shr, ShrAssign, Rem, RemAssign, BitOrAssign, BitXor, Not, Sub, BitAnd, BitOr};
use std::fmt::Debug;

use num::Num;
use num::FromPrimitive;

use vec_scope::*;
use bit_vec_scope::*;
use bit_word_scope::*;
use shape::*;


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


/*
/* Bit Word Scope */

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
struct PackedBitScope {
    bytes: Vec<u8>,
    pos: usize,
    bits_used: u8,
}

impl PackedBitScope {
    fn with_words(bytes: Vec<u8>, bits_used: u8) -> PackedBitScope {
        PackedBitScope {
            bytes: bytes,
            pos: 0,
            bits_used: bits_used,
        }
    }
}

impl Lens<bool> for PackedBitScope {
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

/*
impl Lens<u32> for PackedBitScope {
    fn get(&self) -> u32 {
        let index = self.pos / 8;
        let bit_index = self.pos % 8;
        (self.bytes[index] & (1 << bit_index)) != 0
    }

    fn set(&mut self, a: u32) {
        let index = self.pos / 8;
        let bit_index = self.pos % 8;
        self.bytes[index] = (self.bytes[index] & !(1 << bit_index)) | ((a as u8) << bit_index);
    }
}

impl Scope<u32, usize> for PackedBitScope {
    fn adjust(&mut self, pos: usize) {
        self.pos = clamp(pos, 0, (self.bytes.len() * 8) - 1);
    }
}

impl Scope<u32, isize> for PackedBitScope {
    fn adjust(&mut self, offset: isize) {
        self.pos = clamp((self.pos as isize) + offset, 0, ((8 * self.bytes.len()) - 1) as isize) as usize;
    }
}
*/
*/
