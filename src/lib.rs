use std::ops::{BitAnd, Not, Shl, Shr, BitOr};


trait Scope<A> {
    fn get(&self) -> A;

    fn set(&mut self, a: A);

    fn modify<F: Fn(A) -> A>(&mut self, f: F) {
        self.set(f(self.get()));
    }
}

trait AbsScope<A>: Scope<A> {
    fn set_pos(&mut self, pos: usize);
}

trait RelScope<A>: Scope<A> {
    fn move_pos(&mut self, offset: isize);
}


/* Vec Scope */
#[derive(Clone, PartialEq, Eq)]
struct VecScope<A> {
    vec: Vec<A>,
    pos: usize,
}

impl<A: Copy> Scope<A> for VecScope<A> {
    fn get(&self) -> A {
        self.vec[self.pos]
    }

    fn set(&mut self, a: A) {
        self.vec[self.pos] = a;
    }
}

impl <A: Copy> AbsScope<A> for VecScope<A> {
    fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
}

impl <A: Copy> RelScope<A> for VecScope<A> {
    fn move_pos(&mut self, offset: isize) {
        self.pos = ((self.pos as isize) + offset) as usize;
    }
}


/* Bit Vec Scope */
type BitVecScope = VecScope<u32>;

impl Scope<bool> for BitVecScope {
    fn get(&self) -> bool {
        let index = self.pos / 32;
        let bit_index = self.pos % 32;
        (self.vec[index] & (1 << bit_index)) != 0
    }

    fn set(&mut self, a: bool) {
        let index = self.pos / 32;
        let bit_index = self.pos % 32;
        self.vec[index] = (self.vec[index] & !(1 << bit_index)) | ((a as u32) << bit_index);
    }
}

impl AbsScope<bool> for BitVecScope {
    fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
}

impl RelScope<bool> for BitVecScope {
    fn move_pos(&mut self, offset: isize) {
        self.pos = ((self.pos as isize) + offset) as usize;
    }
}


/* Bit Word Scope */
struct BitWordScope {
    vec: Vec<u32>,
    bits_used: usize,
    pos: usize,
}

impl Scope<bool> for BitWordScope {
    fn get(&self) -> bool {
        let index = self.pos / self.bits_used;
        let bit_index = self.pos % self.bits_used;
        (self.vec[index] & (1 << bit_index)) != 0
    }

    fn set(&mut self, a: bool) {
        let index = self.pos / self.bits_used;
        let bit_index = self.pos % self.bits_used;
        self.vec[index] = (self.vec[index] & !(1 << bit_index)) | ((a as u32) << bit_index);
    }
}


/* Word Scope */
struct WordScope<B> {
    vec: Vec<B>,
    bits_used: usize,
    pos: usize,
}

impl<B: Copy> Scope<B> for WordScope<B> {
    fn get(&self) -> B {
        self.vec[self.pos / self.bits_used]
    }

    fn set(&mut self, a: B) {
        let index = self.pos / self.bits_used;
        self.vec[index] = a;
    }
}

impl<A: Copy> AbsScope<A> for WordScope<A> {
    fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
}

impl<A: Copy> RelScope<A> for WordScope<A> {
    fn move_pos(&mut self, offset: isize) {
        self.pos = ((self.pos as isize) + offset) as usize;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

