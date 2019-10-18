use num::clamp;
use std::rc::Rc;

use myopic::lens::lens::*;
use myopic::lens::*;

use crate::scope::*;
use crate::shape::*;


/* Vec Scope */
#[derive(Clone, PartialEq, Eq)]
pub struct VecScope<A> {
    pub vec: Vec<A>,
    pub pos: usize,
}

impl<A> VecScope<A> {
    pub fn with_vec(vec: Vec<A>) -> Option<VecScope<A>> {
        if vec.len() > 0 {
            Some(VecScope {
                vec: vec,
                pos: 0,
            })
        } else {
            None
        }
    }
}

impl<A> Shape for VecScope<A> {
    type Shape = usize;

    fn shape(&self) -> usize {
        self.vec.len()
    }
}

impl<A: Copy> VecScope<A> {
    pub fn lens() -> impl Optical<Input=VecScope<A>, Output=A> {
        let lens: Lens<_, _, VecScope<A>, A> =
            Lens::new(|vec: &VecScope<A>| get_vec_scope(vec),
                      |vec: &mut VecScope<A>, a: A| set_vec_scope(vec, a));

        return lens;
    }
}

pub fn get_vec_scope<A: Copy>(vec_scope: &VecScope<A>) -> A {
    vec_scope.vec[vec_scope.pos]
}

pub fn set_vec_scope<A>(vec_scope: &mut VecScope<A>, a: A) {
    vec_scope.vec[vec_scope.pos] = a;
}

impl <A: Copy> Scope<usize> for VecScope<A> {
    fn adjust(&mut self, pos: usize) {
        self.pos = clamp(pos, 0, self.vec.len() - 1);
    }
}

impl <A: Copy> Scope<isize> for VecScope<A> {
    fn adjust(&mut self, offset: isize) {
        self.pos = clamp((self.pos as isize) + offset, 0, (self.vec.len() - 1) as isize) as usize;
    }
}

#[test]
fn test_vec_scope() {
    let mut vec_scope: VecScope<usize> = VecScope::with_vec(vec![1,2,3,4,5]).unwrap();

    let vec_lens = VecScope::lens();

    assert_eq!(get_vec_scope(&vec_scope), 1);

    vec_lens.set(&mut vec_scope, 100);
    assert_eq!(get_vec_scope(&vec_scope), 100);

    vec_scope.adjust(1isize);
    assert_eq!(get_vec_scope(&vec_scope), 2);

    vec_scope.adjust(2usize);
    assert_eq!(get_vec_scope(&vec_scope), 3);

    vec_scope.adjust(3usize);
    assert_eq!(get_vec_scope(&vec_scope), 4);

    vec_scope.adjust(100usize);
    assert_eq!(get_vec_scope(&vec_scope), 5);

    vec_scope.adjust(-1isize);
    assert_eq!(get_vec_scope(&vec_scope), 4);

    vec_scope.adjust(100isize);
    assert_eq!(get_vec_scope(&vec_scope), 5);

    vec_lens.set(&mut vec_scope, 500);
    assert_eq!(get_vec_scope(&vec_scope), 500);

    vec_scope.adjust(-100isize);
    assert_eq!(get_vec_scope(&vec_scope), 100);
}

#[test]
fn test_vec_scope_fields() {
    let vec_lens = VecScope::lens();

    let pair_lens = 
        Lens::new(|pair: &(usize, isize)| pair.0,
                  |pair: &mut (usize, isize), a: usize| pair.0 = a);

    let vec_pair_lens = ComposedLens::new(vec_lens, pair_lens);

    let mut vec_scope = VecScope::with_vec(vec![(1, 2), (3, 4)]).unwrap();

    assert_eq!(vec_scope.vec[0].0, 1);

    vec_pair_lens.set(&mut vec_scope, 100);
    assert_eq!(vec_scope.vec[0].0, 100);
    assert_eq!(vec_pair_lens.get(&vec_scope), 100);


    vec_scope.adjust(1isize);
    assert_eq!(vec_scope.vec[1].0, 3);
    assert_eq!(vec_pair_lens.get(&vec_scope), 3);
}

#[test]
fn vec_scope_seq() {
    let length = 100_000_000;
    let step = 100;
    let times = 10;

    let mut indices: Vec<u32> = Vec::with_capacity(length);
    let mut vec_scope = VecScope::with_vec(vec!(0; length)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform2 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    for _ in 0..times {
        transform1.transform(&mut vec_scope);
        transform2.transform(&mut vec_scope);
    }
}

#[test]
fn vec_scope_single() {
    let length = 100_000_000;
    let step = 100;
    let times = 10;

    let mut indices: Vec<u32> = Vec::with_capacity(length);
    let mut vec_scope = VecScope::with_vec(vec!(0; length)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    for _ in 0..times {
        transform1.transform(&mut vec_scope);
    }
}

#[test]
fn vec_scope_simple_1() {
    let length = 100_000_000;
    let step = 100;
    let times = 1;

    let mut vec = vec!(0; length);

    for _ in 0..times {
        for index in (0..length).step_by(step) {
            vec[index] += 1;
        }
    }
}

#[test]
fn vec_scope_simple_2() {
    let length = 100_000_000;
    let step = 100;
    let times = 2;

    let mut vec = vec!(0; length);

    for _ in 0..times {
        for index in (0..length).step_by(step) {
            vec[index] += 1;
        }
    }
}

#[test]
fn vec_scope_simple_10() {
    let length = 100_000_000;
    let step = 100;
    let times = 10;

    let mut vec = vec!(0; length);

    for _ in 0..times {
        for index in (0..length).step_by(step) {
            vec[index] += 1;
        }
    }
}

#[test]
fn vec_scope_both() {
    let length = 100_000_000;
    let step = 100;
    let times = 10;

    let mut vec_scope = VecScope::with_vec(vec!(0; length)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform2 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    for _ in 0..times {
        apply_both(&transform1, &transform2, &mut vec_scope);
    }
}

/*
#[test]
fn vec_scope_many_2() {
    let length = 100_000_000;
    let step = 100;
    let times = 10;

    let mut vec_scope = VecScope::with_vec(vec!(0; length)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform2 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transforms = vec!(transform1, transform2);
    apply_many(transforms, &mut vec_scope);
}

#[test]
fn vec_scope_many_3() {
    let length = 100_000_000;
    let step = 100;
    let times = 10;

    let mut vec_scope = VecScope::with_vec(vec!(0; length)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform2 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform3 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transforms = vec!(transform1, transform2, transform3);
    apply_many(transforms, &mut vec_scope);
}

#[test]
fn vec_scope_many_4() {
    let length = 100_000_000;
    let step = 100;
    let times = 10;

    let mut vec_scope = VecScope::with_vec(vec!(0; length)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform2 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform3 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform4 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transforms = vec!(transform1, transform2, transform3, transform4);
    apply_many(transforms, &mut vec_scope);
}

#[test]
fn vec_scope_many_5() {
    let length = 100_000_000;
    let step = 100;
    let times = 10;

    let mut vec_scope = VecScope::with_vec(vec!(0; length)).unwrap();
    let transform1 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform2 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform3 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform4 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transform5 =
        Transform::make_transform(VecScope::lens(),
                                  (0..length).step_by(step),
                                  |val| val + 1);

    let transforms = vec!(transform1, transform2, transform3, transform4, transform5);
    apply_many(transforms, &mut vec_scope);
}
*/
