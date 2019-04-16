use std::rc::Rc;

use crate::shape::*;
use crate::lens::*;


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

