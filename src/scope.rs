
use std::iter::*;
use std::boxed::*;

use num::{PrimInt, zero, one};

use crate::lens::*;
use crate::shape::*;


// NOTE this is just a Monoid Action
pub trait Scope<I> {
    fn adjust(&mut self, index: I);
}

pub struct Action<A, Ix> {
    pub act: Box<Fn(A) -> A>,
    pub indices: Ix,
}

pub fn scope_map<I, S, A, F>(s: &mut S, function: Box<Fn(A) -> A>, lens: Lens<S, A>)
    where S: Scope<I> + Shape<Shape=I>,
          I: PrimInt {
    let cap = s.shape();
    let mut index = zero();
    while index != cap {
        s.adjust(index);

        let val = (lens.view)(s);
        let val_f = (function)(val);
        (lens.set)(s, val_f);

        index = index + one();
    }
}

pub fn scope_ixmap<I, S, Ix, A, F>(s: &mut S, action: Action<A, Ix>, lens: Lens<S, A>)
    where S: Scope<I>,
          Ix: Iterator<Item=I> {
    for index in action.indices {
        s.adjust(index);

        let val = (lens.view)(s);
        let val_f = (action.act)(val);
        (lens.set)(s, val_f);
    }
}

