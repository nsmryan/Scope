use std::rc::Rc;
use std::ops::{Shl, ShlAssign, Shr, ShrAssign, Rem, RemAssign, BitOrAssign, BitXor, Not, Sub, BitAnd, BitOr};

use num::clamp;

use crate::shape::*;
use crate::lens::*;
use crate::types::*;
use crate::scope::*;


#[derive(Clone, PartialEq, Eq)]
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

impl<B: BitWise> BitWordScope<B> {
    fn bit_lens() -> Lens<BitWordScope<B>, bool> {
        lens(Rc::new(|vec: &BitWordScope<B>| get_bitword_scope_bits(vec)),
             Rc::new(|mut vec: &mut BitWordScope<B>, a: bool| set_bitword_scope_bits(vec, a)))
    }
}

impl<B: BitWise> BitWordScope<B> {
    fn lens() -> Lens<BitWordScope<B>, B> {
        lens(Rc::new(|vec: &BitWordScope<B>| get_bitword_scope(vec)),
             Rc::new(|mut vec: &mut BitWordScope<B>, a: B| set_bitword_scope(vec, a)))
    }
}

impl<B> Shape for BitWordScope<B> {
    type Shape = usize;

    fn shape(&self) -> usize {
        self.vec.len()
    }
}

fn get_bitword_scope_bits<B: BitWise>(bitword_scope: &BitWordScope<B>) -> bool {
    let index = bitword_scope.pos / bitword_scope.bits_used;
    let bit_index = (bitword_scope.pos % bitword_scope.bits_used) as u32;
    (bitword_scope.vec[index] & (B::one() << bit_index)) != B::zero()
}

fn set_bitword_scope_bits<B: BitWise>(bitword_scope: &mut BitWordScope<B>, a: bool) {
    let index = bitword_scope.pos / bitword_scope.bits_used;
    let bit_index = (bitword_scope.pos % bitword_scope.bits_used) as u32;

    let loc_cleared = bitword_scope.vec[index] & !(B::one() << bit_index);
    let set_bit = B::from_u8(a as u8).unwrap() << bit_index;
    let loc_set = loc_cleared | set_bit;
    bitword_scope.vec[index] = loc_set;
}

fn get_bitword_scope<B: BitWise>(bitword_scope: &BitWordScope<B>) -> B {
        let index = bitword_scope.pos / bitword_scope.bits_used;
        bitword_scope.vec[index]
}

fn set_bitword_scope<B: BitWise>(bitword_scope: &mut BitWordScope<B>, a: B) {
        let index = bitword_scope.pos / bitword_scope.bits_used;
        bitword_scope.vec[index] = a;
}

impl<B> Scope<usize> for BitWordScope<B> {
    fn adjust(&mut self, pos: usize) {
        self.pos = clamp(pos, 0, (self.vec.len() * self.bits_used) - 1);
    }
}

impl<B> Scope<isize> for BitWordScope<B> {
    fn adjust(&mut self, offset: isize) {
        self.pos = clamp((self.pos as isize) + offset, 0, ((self.bits_used * self.vec.len()) - 1) as isize) as usize;
    }
}

#[test]
fn test_bit_word_scope() {
    let mut bit_word_scope: BitWordScope<u8> = BitWordScope::with_words(vec![1,2,3,4,0x7], 3);

    let lens = BitWordScope::lens();

    let current: u8 = (lens.view)(&bit_word_scope);
    assert_eq!(current, 1);

    let current: u8 = (lens.view)(&bit_word_scope);
    assert_eq!(current, 1);

    (lens.set)(&mut bit_word_scope, 100);
    let current: u8 = (lens.view)(&bit_word_scope);
    assert_eq!(current, 100);

    bit_word_scope.adjust(1usize);
    (lens.set)(&mut bit_word_scope, 100);
    assert_eq!(bit_word_scope.vec[0], 100);

    bit_word_scope.adjust(100isize);
    let current: u8 = (lens.view)(&bit_word_scope);
    assert_eq!(current, 7);

    bit_word_scope.adjust(100usize);
    let current: u8 = (lens.view)(&bit_word_scope);
    assert_eq!(current, 0x07);
}
