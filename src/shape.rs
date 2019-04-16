

pub trait Shape {
    type Shape;

    fn shape(&self) -> Self::Shape;
}

impl<A> Shape for Vec<A> {
    type Shape = usize;

    fn shape(&self) -> usize {
        self.len()
    }
}
