use std::marker::PhantomData;
use std::iter::*;
use std::collections::VecDeque;
use std::cmp::Ordering;

use num::{PrimInt, zero, one};

use myopic::*;
use myopic::lens::lens::*;

use crate::shape::*;


// NOTE this is just a Monoid Action
pub trait Scope<I> {
    fn adjust(&mut self, index: I);
}

pub struct Action<F, O, D, A> {
    pub act: F,
    pub lens: O,
    a: PhantomData<A>,
    d: PhantomData<D>,
}

impl<F, O, D, A> Action<F, O, D, A> 
    where O: Getter + Setter + Lensable<Input=D, Output=A>,
          F: Fn(A) -> A {
    pub fn act(&self, d: &mut D) {
        let val = self.lens.get(d);
        let val_f = (self.act)(val);
        self.lens.set(d, val_f);
    }
}

pub struct Transform<F, O, D, A, Ix> {
    pub action: Action<F, O, D, A>,
    pub indices: Ix,
}

impl<F, O, D, I, A, Ix> Transform<F, O, D, A, Ix> 
    where O: Getter + Setter + Lensable<Input=D, Output=A>,
          D: Shape + Scope<I>,
          F: Fn(A) -> A,
          Ix: Clone + IntoIterator<Item=I> {
    pub fn transform(&self, d: &mut D) {
        for index in self.indices.clone().into_iter() {
            d.adjust(index);

            self.action.act(d);
        }
    }

    pub fn make_transform(lens: O, indices: Ix, f: F) -> Transform<F, O, D, A, Ix>
        where F: Fn(A) -> A + 'static {
        Transform {
            action: Action {
                act: f,
                lens: lens,
                d: PhantomData,
                a: PhantomData,
            },

            indices: indices,
        }
    }
}

pub fn apply_both<F, G, O, D, A, Ix, I>(first: &Transform<F, O, D, A, Ix>,
                                        second: &Transform<G, O, D, A, Ix>,
                                        d: &mut D) 
    where D: Scope<I> + Shape,
          Ix: IntoIterator<Item=I> + Clone,
          I: PartialOrd + Copy,
          F: Fn(&mut D),
          G: Fn(&mut D),
          O: Getter + Setter + Lensable<Input=D, Output=A> {

    let mut ixs1 = first.indices.clone().into_iter();
    let mut ixs2 = second.indices.clone().into_iter();

    if let (Some(mut ix1), Some(mut ix2)) = (ixs1.next(), ixs2.next()) {
        loop {
            if ix1 < ix2 {
                d.adjust(ix1);
                (first.action.act)(d);

                if let Some(new_ix) = ixs1.next() {
                    ix1 = new_ix;
                } else {
                    break;
                }
            } else {
                d.adjust(ix2);
                (second.action.act)(d);

                if let Some(new_ix) = ixs2.next() {
                    ix2 = new_ix;
                } else {
                    break;
                }
            }
        }
    }

    for index in ixs1 {
        d.adjust(index);
        (first.action.act)(d);
    }

    for index in ixs2 {
        d.adjust(index);
        (second.action.act)(d);
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

pub fn apply_many<F, O, D, A, Ix, I>(transforms: Vec<Transform<F, O, D, A, Ix>>, d: &mut D) 
    where D: Scope<I> + Shape,
          Ix: IntoIterator<Item=I> + Clone,
          I: PartialOrd + Ord + Copy,
          F: Fn(A) -> A,
          O: Getter + Setter + Lensable<Input=D, Output=A> {

    let mut pqueue = VecDeque::new();
    let mut ix_vec = vec!();

    for index in 0..transforms.len() {
        ix_vec.push(transforms[index].indices.clone().into_iter());
        if let Some(ix) = ix_vec[index].next() {
            pqueue.push_back(Queued { ix: ix, index: index });
        }
    }

    while let Some(queued) = pqueue.pop_front() {
        d.adjust(queued.ix);
        transforms[queued.index].action.act(d);
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

pub fn scope_map<I, O, D, A, F>(d: &mut D, action: Action<F, O, D, A>)
    where D: Scope<I> + Shape<Shape=I>,
          I: PrimInt,
          F: Fn(A) -> A,
          O: Getter + Setter + Lensable<Input=D, Output=A> {
    let cap = d.shape();
    let mut index = zero();
    while index != cap {
        d.adjust(index);
        action.act(d);
        index = index + one();
    }
}

pub fn scope_ixmap<F, O, I, Ix, D, A>(d: &mut D, ix: Ix, action: Action<F, O, D, A>)
    where D: Scope<I>,
          Ix: Iterator<Item=I>,
          F: Fn(&mut D),
          O: Getter + Setter + Lensable<Input=D, Output=A> {
    for index in ix {
        d.adjust(index);
        (action.act)(d);
    }
}

