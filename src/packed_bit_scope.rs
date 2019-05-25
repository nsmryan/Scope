use std::rc::Rc;
use std::cmp;

use num::clamp;

use num::zero;
use num::PrimInt;
use num::cast::NumCast;

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
        let max_pos = (self.bytes.len() * 8) / self.bits_used;
        self.pos = clamp(pos, 0, max_pos);
    }
}

impl Scope<isize> for PackedBitScope {
    fn adjust(&mut self, offset: isize) {
        let max_pos = (self.bytes.len() * 8) / self.bits_used;
        self.pos = clamp((self.pos as isize) + offset, 0, max_pos as isize) as usize;
    }
}

impl PackedBitScope {
    pub fn bit_lens() -> Lens<PackedBitScope, bool> {
        lens(Rc::new(|bytes: &PackedBitScope| get_packedbit_scope_bits(bytes)),
             Rc::new(|mut bytes: &mut PackedBitScope, a: bool| set_packedbit_scope_bits(bytes, a)))
    }

    pub fn num_lens<N: PrimInt>() -> Lens<PackedBitScope, N> {
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

pub fn get_packedbit_scope_num<N: PrimInt>(packedbit_scope: &PackedBitScope) -> N {
    let bit_pos = packedbit_scope.pos * packedbit_scope.bits_used;
    let mut index = bit_pos / 8;
    let bit_index = (bit_pos % 8) as u32;

    let mut bits_left = packedbit_scope.bits_used;
    let mut bits_found = 0;

    let mut n: N = zero();

    // first bits
    if bit_index != 0 {
        let bits: N = NumCast::from(packedbit_scope.bytes[index] >> bit_index).unwrap();
        bits_found += cmp::min(8 - bit_index as usize, packedbit_scope.bits_used);
        n = n | (bits & NumCast::from(2u32.pow(bits_found as u32) - 1).unwrap());
        bits_left -= bits_found;
        index += 1;
    }

    // middle bits
    while bits_left >= 8 {
        let bits: N = NumCast::from(packedbit_scope.bytes[index]).unwrap();
        n = n | (bits << bits_found);
        bits_found += 8;
        bits_left -= 8;
        index += 1;
    }

    // final bits
    if bits_left > 0 {
        let bits = packedbit_scope.bytes[index] & (2u8.pow(bits_left as u32) - 1);
        let bits: N = NumCast::from(bits).unwrap();
        n = n | (bits << bits_found);
    }

    return n;
}

pub fn set_packedbit_scope_num<N: PrimInt>(packedbit_scope: &mut PackedBitScope, n: N) {
    let bit_pos = packedbit_scope.pos * packedbit_scope.bits_used;
    let mut index = bit_pos / 8;
    let bit_index = (bit_pos % 8) as u32;

    let mut bits_left = packedbit_scope.bits_used;
    let mut bits_used = 0;

    // first byte
    if bit_index != 0 {
        let mut first_byte: u8 =
            packedbit_scope.bytes[index] as u8 & (2u8.pow(bit_index) - 1);
        first_byte |= n.to_u8().unwrap() << bit_index as usize;
        packedbit_scope.bytes[index] = first_byte;

        bits_used = cmp::min(8 - bit_index as usize, packedbit_scope.bits_used);
        bits_left -= bits_used;
        index += 1;
    }

    // middle bytes
    for _ in 0..(bits_left / 8) {
        packedbit_scope.bytes[index] = (n >> bits_used).to_u8().unwrap();
        index += 1;
        bits_used += 8;
        bits_left -= 8;
    }

    // last byte
    if bits_left > 0 {
        let high_bits = packedbit_scope.bytes[index] & !(2u8.pow(bits_left as u32) - 1);
        packedbit_scope.bytes[index] = high_bits | (n >> bits_used).to_u8().unwrap();
    }
}

#[test]
fn test_packedbit_scope_get_2() {
    let length = 1;
    let mut packed_scope = PackedBitScope::with_words(vec!(0xA5; length), 2);
    let packed_lens = PackedBitScope::num_lens::<u8>();

    assert_eq!((packed_lens.view)(&packed_scope), 0x01);
    packed_scope.adjust(1isize);
    assert_eq!((packed_lens.view)(&packed_scope), 0x01);
    packed_scope.adjust(1isize);
    assert_eq!((packed_lens.view)(&packed_scope), 0x02);
    packed_scope.adjust(1isize);
    assert_eq!((packed_lens.view)(&packed_scope), 0x02);
}

#[test]
fn test_packedbit_scope_get_9() {
    let length = 4;
    let mut packed_scope = PackedBitScope::with_words(vec!(0x11; length), 9);
    let packed_lens = PackedBitScope::num_lens::<u16>();

    assert_eq!((packed_lens.view)(&packed_scope), 0x111);
    packed_scope.adjust(1isize);
    assert_eq!((packed_lens.view)(&packed_scope), 0x088);
    packed_scope.adjust(1isize);
    assert_eq!((packed_lens.view)(&packed_scope), 0x044);
}

#[test]
fn test_packedbit_scope_set_9() {
    let length = 4;
    let mut packed_scope = PackedBitScope::with_words(vec!(0x11; length), 9);
    let packed_lens = PackedBitScope::num_lens::<u16>();

    (packed_lens.set)(&mut packed_scope, 0xA5);
    assert_eq!((packed_lens.view)(&packed_scope), 0xA5);

    packed_scope.adjust(1isize);
    (packed_lens.set)(&mut packed_scope, 0x5A);
    assert_eq!((packed_lens.view)(&packed_scope), 0x5a);

    packed_scope.adjust(-1isize);
    assert_eq!((packed_lens.view)(&packed_scope), 0xA5);
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
