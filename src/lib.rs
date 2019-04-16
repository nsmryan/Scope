extern crate num;

mod lens;
mod scope;
mod vec_scope;
mod bit_vec_scope;
mod bit_word_scope;
mod shape;
mod types;


use num::Num;

//use vec_scope::*;
//use bit_vec_scope::*;
//use bit_word_scope::*;
//use shape::*;



/*
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
