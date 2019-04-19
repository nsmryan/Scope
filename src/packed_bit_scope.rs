use std::rc::Rc;
use std::io::{Read, Write, Cursor};

use bitstream_io::*;
use bitstream_io::read::*;
use bitstream_io::write::*;

use crate::lens::*;
use crate::shape::*;


#[derive(Clone, PartialEq, Eq)]
struct PackedBitScope {
    bytes: Vec<u8>,
    pos: usize,
    bits_used: u8,
}

impl Shape for PackedBitScope {
    type Shape = usize;

    fn shape(&self) -> usize {
        self.bytes.len() / self.bits_used as usize
    }
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

impl PackedBitScope {
    fn bit_lens() -> Lens<PackedBitScope, bool> {
        lens(Rc::new(|bytes: &PackedBitScope| get_packedbit_scope_bits(bytes)),
             Rc::new(|mut bytes: &mut PackedBitScope, a: bool| set_packedbit_scope_bits(bytes, a)))
    }

    /*
    fn bit_lens<B>() -> Lens<PackedBitScope, B> {
        lens(Rc::new(|bytes: &PackedBitScope<B>| get_packedbit_scope_bits(bytes)),
             Rc::new(|mut bytes: &mut PackedBitScope<B>, a: bool| set_packedbit_scope_bits(bytes, a)))
    }
    */
}

fn get_packedbit_scope_bits(packedbit_scope: &PackedBitScope) -> bool {
    let index = packedbit_scope.pos / 8;
    let bit_index = (packedbit_scope.pos % 8) as u32;
    (packedbit_scope.bytes[index] & (1 << bit_index)) != 0
}

fn set_packedbit_scope_bits(packedbit_scope: &mut PackedBitScope, a: bool) {
    let index = packedbit_scope.pos / 8;
    let bit_index = (packedbit_scope.pos % 8) as u32;

    let loc_cleared = packedbit_scope.bytes[index] & !(1 << bit_index);
    let set_bit = (a as u8) << bit_index;
    let loc_set = loc_cleared | set_bit;
    packedbit_scope.bytes[index] = loc_set;
}

fn get_packedbit_scope_num<N: Numeric>(packedbit_scope: &PackedBitScope) -> N {
    let index = packedbit_scope.pos / 8;
    let bit_index = (packedbit_scope.pos % 8) as u32;

    let mut reader = BitReader::endian(Cursor::new(&packedbit_scope.bytes[index..]), BigEndian);

    reader.skip(bit_index);
    reader.read(N::bits_size()).unwrap()
}

fn set_packedbit_scope_num<N: Numeric>(packedbit_scope: &PackedBitScope, n: N) {
    let index = packedbit_scope.pos / 8;
    let bit_index = (packedbit_scope.pos % 8) as u32;

    let mut writer = BitWriter::endian(&mut packedbit_scope.bytes[index..].to_vec(), BigEndian);

    writer.skip(bit_index);
    writer.write(N::bits_size(), n);
}

