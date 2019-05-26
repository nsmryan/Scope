use num::clamp;
use std::rc::Rc;

use crate::lens::*;
use crate::scope::*;
use crate::shape::*;


#[derive(Clone, PartialEq, Eq)]
pub struct BitVecScope {
    pub bytes: Vec<u8>,
    pub pos: usize,
}

impl Shape for BitVecScope {
    type Shape = usize;

    fn shape(&self) -> usize {
        self.bytes.len() * 8
    }
}

impl BitVecScope {
    pub fn with_bytes(bytes: Vec<u8>) -> Option<BitVecScope> {
        if bytes.len() > 0 {
            Some(BitVecScope {
                bytes: bytes,
                pos: 0,
            })
        } else {
            None
        }
    }

    pub fn lens() -> Lens<BitVecScope, bool> {
        lens(Rc::new(|vec: &BitVecScope| get_vec_scope(vec)),
             Rc::new(|vec: &mut BitVecScope, a: bool| set_vec_scope(vec, a)))
    }

    pub fn byte_index(&self) -> usize {
        self.pos / 8
    }

    pub fn current_byte(&self) -> u8 {
        self.bytes[self.byte_index()]
    }
}

pub fn get_vec_scope(bit_vec_scope: &BitVecScope) -> bool {
    let bit_index = bit_vec_scope.pos % 8;
    (bit_vec_scope.current_byte() & (1 << bit_index)) != 0
}

pub fn set_vec_scope(bit_vec_scope: &mut BitVecScope, a: bool) {
    let bit_index = bit_vec_scope.pos % 8;
    let index = bit_vec_scope.byte_index();
    bit_vec_scope.bytes[index] =
        (bit_vec_scope.current_byte() & !(1 << bit_index)) | ((a as u8) << bit_index);
}

impl Scope<usize> for BitVecScope {
    fn adjust(&mut self, pos: usize) {
        self.pos = clamp(pos, 0, (self.bytes.len() * 8) - 1);
    }
}

impl Scope<isize> for BitVecScope {
    fn adjust(&mut self, offset: isize) {
        self.pos = clamp((self.pos as isize) + offset, 0, ((8 * self.bytes.len()) - 1) as isize) as usize;
    }
}

#[test]
fn test_bit_vec_scope() {
    let mut bit_vec_scope = BitVecScope::with_bytes(vec![1,2,3,4,0x80]).unwrap();

    let lens = BitVecScope::lens();

    assert_eq!((lens.view)(&bit_vec_scope), true);

    (lens.set)(&mut bit_vec_scope, false);
    assert_eq!((lens.view)(&bit_vec_scope), false);

    bit_vec_scope.adjust(1usize);
    (lens.set)(&mut bit_vec_scope, true);
    assert_eq!(bit_vec_scope.bytes[0], 0x02);

    bit_vec_scope.adjust(100isize);
    assert_eq!((lens.view)(&bit_vec_scope), true);
}
