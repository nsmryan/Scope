
use std::iter::*;
use std::boxed::*;

use num::{PrimInt, zero, one};

use crate::lens::*;
use crate::shape::*;


// NOTE this is just a Monoid Action
pub trait Scope<I> {
    fn adjust(&mut self, index: I);
}

pub struct Action<S, A> {
    pub act: Box<Fn(A) -> A>,
    pub lens: Lens<S, A>,
}

impl<S, A> Action<S, A> {
    pub fn act(&self, s: &mut S) {
        let val = (self.lens.view)(s);
        let val_f = (self.act)(val);
        (self.lens.set)(s, val_f);
    }
}

pub struct Transform<S, A, Ix> {
    pub action: Action<S, A>,
    pub indices: Ix,
}

impl<S: Shape + Scope<I>, I, A, Ix: Clone + IntoIterator<Item=I>> Transform<S, A, Ix> {
    pub fn transform(&self, s: &mut S) {
        for index in self.indices.clone().into_iter() {
            s.adjust(index);

            self.action.act(s);
        }
    }
}

pub fn apply_both<S, A, Ix, I>(first: &Transform<S, A, Ix>,
                               second: &Transform<S, A, Ix>,
                               s: &mut S) 
    where S: Scope<I> + Shape,
          Ix: IntoIterator<Item=I> + Clone,
          I: PartialOrd + Copy {

    let mut ixs1 = first.indices.clone().into_iter().peekable();
    let mut ixs2 = second.indices.clone().into_iter().peekable();

    loop {
        match (ixs1.peek(), ixs2.peek()) {
            (Some(ix1), Some(ix2)) => {
                if *ix1 < *ix2 {
                    s.adjust(*ix1);
                    first.action.act(s);
                    ixs1.next();
                } else if *ix1 == *ix2 {
                    s.adjust(*ix1);
                    first.action.act(s);
                    second.action.act(s);
                    ixs1.next();
                    ixs2.next();
                } else {
                    s.adjust(*ix2);
                    second.action.act(s);
                    ixs2.next();
                }
            },

            (Some(ix1), None) => {
                s.adjust(*ix1);
                first.action.act(s);
                ixs1.next();
            },

            (None, Some(ix2)) => {
                s.adjust(*ix2);
                second.action.act(s);
                ixs2.next();
            },

            (None, None) => {
                break;
            }
        }
    }
}

pub fn scope_map<I, S, A, F>(s: &mut S, action: Action<S, A>)
    where S: Scope<I> + Shape<Shape=I>,
          I: PrimInt {
    let cap = s.shape();
    let mut index = zero();
    while index != cap {
        s.adjust(index);

        action.act(s);

        index = index + one();
    }
}

pub fn scope_ixmap<I, S, Ix, A, F>(s: &mut S, ix: Ix, action: Action<S, A>)
    where S: Scope<I>,
          Ix: Iterator<Item=I> {
    for index in ix {
        s.adjust(index);

        action.act(s);
    }
}

