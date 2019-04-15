

// NOTE this is just a Moniod Action
pub trait Scope<I> {
    fn adjust(&mut self, index: I);
}

