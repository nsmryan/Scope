
use std::iter::*;
use std::boxed::*;
use std::collections::binary_heap::*;
use std::collections::VecDeque;
use std::cmp::Ordering;

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

    pub fn make_transform<F>(lens: Lens<S, A>, indices: Ix, f: F) -> Transform<S, A, Ix>
        where F: Fn(A) -> A + 'static {
        Transform {
            action: Action {
                act: Box::new(f),
                lens: lens,
            },

            indices: indices,
        }
    }
}

pub fn apply_both<S, A, Ix, I>(first: &Transform<S, A, Ix>,
                               second: &Transform<S, A, Ix>,
                               s: &mut S) 
    where S: Scope<I> + Shape,
          Ix: IntoIterator<Item=I> + Clone,
          I: PartialOrd + Copy {

    let mut ixs1 = first.indices.clone().into_iter();
    let mut ixs2 = second.indices.clone().into_iter();

    if let (Some(mut ix1), Some(mut ix2)) = (ixs1.next(), ixs2.next()) {
        loop {
            if ix1 < ix2 {
                s.adjust(ix1);
                first.action.act(s);

                if let Some(new_ix) = ixs1.next() {
                    ix1 = new_ix;
                } else {
                    break;
                }
            } else {
                s.adjust(ix2);
                second.action.act(s);

                if let Some(new_ix) = ixs2.next() {
                    ix2 = new_ix;
                } else {
                    break;
                }
            }
        }
    }

    for index in ixs1 {
        s.adjust(index);
        first.action.act(s);
    }

    for index in ixs2 {
        s.adjust(index);
        second.action.act(s);
    }
}

#[derive(Eq, PartialEq)]
struct Queued<I> {
    ix: I,
    index: usize,
}

impl<I: Ord> PartialOrd for Queued<I> {
    fn partial_cmp(&self, other: &Queued<I>) -> Option<Ordering> {
        Some(self.ix.cmp(&other.ix))
    }
}

impl<I: Ord> Ord for Queued<I> {
    fn cmp(&self, other: &Queued<I>) -> Ordering {
        self.ix.cmp(&other.ix)
    }
}

pub fn apply_many<S, A, Ix, I>(transforms: Vec<Transform<S, A, Ix>>,
                               s: &mut S) 
    where S: Scope<I> + Shape,
          Ix: IntoIterator<Item=I> + Clone,
          I: PartialOrd + Ord + Copy {

    let mut pqueue = VecDeque::new();
    let mut ix_vec = vec!();

    for index in 0..transforms.len() {
        ix_vec.push(transforms[index].indices.clone().into_iter());
        if let Some(ix) = ix_vec[index].next() {
            pqueue.push_back(Queued { ix: ix, index: index });
        }
    }

    while pqueue.len() > 0 {
        if let Some(queued) = pqueue.pop_front() {
            s.adjust(queued.ix);
            transforms[queued.index].action.act(s);
            if let Some(ix) = ix_vec[queued.index].next() {
                let new_queued = Queued { ix: ix, index: queued.index };

                if let Some(insert_index) = pqueue.iter().position(|queued| queued.ix > ix) {
                    pqueue.insert(insert_index, new_queued);
                } else {
                    pqueue.push_back(new_queued);
                }
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

