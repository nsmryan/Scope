use std::rc::Rc;
use std::io::{Read, Write, Cursor};

use num::clamp;

use bitstream_io::*;
use bitstream_io::read::*;
use bitstream_io::write::*;

use crate::lens::*;
use crate::shape::*;
use crate::scope::*;


#[derive(Clone, PartialEq, Eq)]
pub struct PackedBitScope {
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub bits_used: usize,
}

impl Shape for PackedBitScope {
    type Shape = usize;

    fn shape(&self) -> usize {
        (self.bytes.len() * 8) / self.bits_used
    }
}

impl PackedBitScope {
    pub fn with_words(bytes: Vec<u8>, bits_used: usize) -> PackedBitScope {
        PackedBitScope {
            bytes: bytes,
            pos: 0,
            bits_used: bits_used,
        }
    }
}

impl Scope<usize> for PackedBitScope {
    fn adjust(&mut self, pos: usize) {
        let max_pos = self.bytes.len() - 1;
        self.pos = clamp(pos, 0, max_pos);
    }
}

impl Scope<isize> for PackedBitScope {
    fn adjust(&mut self, offset: isize) {
        let max_pos = self.bytes.len() - 1;
        self.pos = clamp((self.pos as isize) + offset, 0, max_pos as isize) as usize;
    }
}

impl PackedBitScope {
    pub fn bit_lens() -> Lens<PackedBitScope, bool> {
        lens(Rc::new(|bytes: &PackedBitScope| get_packedbit_scope_bits(bytes)),
             Rc::new(|mut bytes: &mut PackedBitScope, a: bool| set_packedbit_scope_bits(bytes, a)))
    }

    pub fn num_lens<N: Numeric>() -> Lens<PackedBitScope, N> {
        lens(Rc::new(|bytes: &PackedBitScope| get_packedbit_scope_num(bytes)),
             Rc::new(|mut bytes: &mut PackedBitScope, n: N| set_packedbit_scope_num(bytes, n)))
    }

    /*
    pub fn stream_lens() -> Lens<PackedBitScope, BitReader<Cursor<&[u8]>, BigEndian>> {
        lens(Rc::new(|bytes: &PackedBitScope| get_packedbit_scope_stream(bytes)),
             Rc::new(|mut bytes: &mut PackedBitScope,
                     bit_reader: BitReader<Cursor<&[u8], BigEndian>| set_packedbit_scope_stream(bytes, n)))
    }
    */
}

pub fn get_packedbit_scope_bits(packedbit_scope: &PackedBitScope) -> bool {
    let bit_pos = packedbit_scope.pos * packedbit_scope.bits_used;
    let index = bit_pos / 8;
    let bit_index = (bit_pos % 8) as u32;
    (packedbit_scope.bytes[index] & (1 << bit_index)) != 0
}

pub fn set_packedbit_scope_bits(packedbit_scope: &mut PackedBitScope, a: bool) {
    let bit_pos = packedbit_scope.pos * packedbit_scope.bits_used;
    let index = bit_pos / 8;
    let bit_index = (bit_pos % 8) as u32;

    let loc_cleared = packedbit_scope.bytes[index] & !(1 << bit_index);
    let set_bit = (a as u8) << bit_index;
    let loc_set = loc_cleared | set_bit;
    packedbit_scope.bytes[index] = loc_set;
}

pub fn get_packedbit_scope_num<N: Numeric>(packedbit_scope: &PackedBitScope) -> N {
    let bit_pos = packedbit_scope.pos * packedbit_scope.bits_used;
    let index = bit_pos / 8;
    let bit_index = (bit_pos % 8) as u32;

    let mut reader = BitReader::endian(Cursor::new(&packedbit_scope.bytes[index..]), BigEndian);

    reader.skip(bit_index).unwrap();
    //reader.read(N::bits_size()).unwrap()
    reader.read(packedbit_scope.bits_used as u32).unwrap()
}

pub fn set_packedbit_scope_num<N: Numeric>(packedbit_scope: &mut PackedBitScope, n: N) {
    let bit_pos = packedbit_scope.pos * packedbit_scope.bits_used;
    let index = bit_pos / 8;
    let bit_index = (bit_pos % 8) as u32;

    let initial_bits = packedbit_scope.bytes[index] as u32 & (2u32.pow(bit_index) - 1);

    let mut writer = BitWriter::endian(&mut packedbit_scope.bytes[index..], BigEndian);

    writer.write(initial_bits, bit_index).unwrap();
    writer.write(packedbit_scope.bits_used as u32, n).unwrap();
}

/*
// TODO consider lens for arbitrary data, providing a bitstream interface for encoding/decoding
pub fn get_packedbit_scope_stream<R: Read, N: Numeric>(packedbit_scope: &PackedBitScope) -> BitReader<Cursor<&[u8]>, BigEndian> {
    let index = packedbit_scope.bit_pos / 8;
    let bit_index = (packedbit_scope.bit_pos % 8) as u32;

    let mut reader = BitReader::endian(Cursor::new(&packedbit_scope.bytes[index..]), BigEndian);

    reader.skip(bit_index);

    reader
}

pub fn set_packedbit_scope_stream<R: Read, N: Numeric>(packedbit_scope: &mut PackedBitScope, bit_reader: &mut BitReader<R, BigEndian>) {
    let index = packedbit_scope.bit_pos / 8;
    let bit_index = (packedbit_scope.bit_pos % 8) as u32;

    let initial_bits = packedbit_scope.bytes[index] as u32 & (2u32.pow(bit_index) - 1);

    let mut writer = BitWriter::endian(&mut packedbit_scope.bytes[index..], BigEndian);

    writer.write(initial_bits, bit_index);

    while !bit_reader.byte_aligned() {
        if let Ok(bit) = bit_reader.read_bit() {
            writer.write_bit(bit);
        } else {
            break;
        }
    }

    while let Ok(byte) = bit_reader.read(8) {
        writer.write(byte, 8);
    }
}
*/
