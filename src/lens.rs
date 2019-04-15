use std::boxed::Box;
use std::rc::Rc;



type Getter<S, A> = Fn(&S) -> A;
type Setter<S, A> = Fn(&mut S, A);

pub struct Lens<S, A> {
    pub view: Rc<Getter<S, A>>,
    pub set: Rc<Setter<S, A>>,
}

pub fn lens<'a, S, A>(getter: Rc<Getter<S, A>>, setter:  Rc<Setter<S, A>>) -> Lens<S, A> {
    Lens {
        view: getter,
        set: setter,
    }
}

pub fn compose<S, A, B>(lhs: Rc<Lens<S, A>>, rhs: Rc<Lens<A, B>>) -> Lens<S, B> 
  where S: 'static, A: 'static, B: 'static {
    let rhs_clone = rhs.clone();
    let lhs_clone = lhs.clone();

    let rhs_clone2 = rhs.clone();
    let lhs_clone2 = lhs.clone();
    lens(Rc::new(move |s| (rhs_clone.view)(&(lhs_clone.view)(s))),

         Rc::new(move |mut s, b| {
             let mut a = (lhs_clone2.view)(s);
             (rhs_clone2.set)(&mut a, b);
             (lhs_clone2.set)(&mut s, a);
         }))
}

