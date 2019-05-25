//use std::ops::{BitOrAssign, Sub, BitAnd, BitOr};

//use bitstream_io::*;

//use num::Zero;


/* BitWise trait for Primitive Types */
// note that this does not require One as the function one() is provided by
// the Numeric type from bitstream_io.
//pub trait BitWise: BitOrAssign<Self> + Sub<Self, Output = Self> +
//                   BitAnd<Output = Self> + BitOr<Output = Self> +
//                   Zero + Numeric {}
//
//impl BitWise for u8  {}
//impl BitWise for u16 {}
//impl BitWise for u32 {}
//impl BitWise for u64 {}
//
//impl BitWise for i8  {}
//impl BitWise for i16 {}
//impl BitWise for i32 {}
//impl BitWise for i64 {}
//
