
use std::iter::*;
use std::boxed::*;

use crate::lens::*;


// NOTE this is just a Monoid Action
pub trait Scope<I> {
    fn adjust(&mut self, index: I);
}

struct Action<A, Ix> {
    act: Box<Fn(A) -> A>,
    indices: Ix,
}

pub fn scope_map<I, S, Ix, A, F>(s: &mut S, ix: Ix, lens: Lens<S, A>, f: F)
    where S: Scope<I>,
          Ix: Iterator<Item=I>,
          F: Fn(A) -> A {
    for index in ix {
        s.adjust(index);

        let val = (lens.view)(s);
        let val_f = (f)(val);
        (lens.set)(s, val_f);
    }
}

