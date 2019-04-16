

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

