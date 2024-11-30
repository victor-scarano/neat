extern crate alloc;
use crate::{node::{Bump, Node}, pop::Pop};
use core::{array, fmt, hash, slice};
use alloc::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    innov: usize,
    bias: f32,
}

impl Input {
    pub fn new_in(innov: usize, bump: Bump) -> Rc<Self, Bump> {
        Pop::next_node_innov();
        Rc::new_in(Self { innov, bias: 0.0 }, bump)
    }

    // we can use self.innov as the idx for any input node
    pub fn index(&self) -> usize {
        self.innov
    }

    pub fn eval<const I: usize>(&self, weight: f32, inputs: [f32; I]) -> f32 {
        weight * (self.bias() + inputs[self.index()])
    }

    pub fn clone_in(&self, bump: Bump) -> Rc<Self, Bump> {
        Rc::new_in(self.clone(), bump)
    }
}

impl Node for Input {
    fn layer(&self) -> usize { 0 }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { panic!(); }
    fn activate(&self, x: f32) -> f32 { panic!(); }
    fn response(&self) -> f32 { panic!(); }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { panic!(); }
}

impl Eq for Input {}

impl hash::Hash for Input {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.innov.hash(state);
    }
}

pub struct Inputs<const I: usize>(Box<[Rc<Input, Bump>; I]>);

impl<const I: usize> Inputs<I> {
    pub fn new_in(bump: Bump) -> Self {
        Self(Box::new(array::from_fn::<_, I, _>(|innov| Input::new_in(innov, bump.clone()))))
    }

    pub fn get(&self, index: usize) -> Option<Rc<Input, Bump>> {
        self.0.get(index).cloned()
    }

    pub fn iter(&self) -> slice::Iter<Rc<Input, Bump>> {
        self.0.iter()
    }
}

impl<const I: usize> fmt::Debug for Inputs<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(&mut f.debug_map(), |f, ref input| {
            f.key_with(|f| fmt::Pointer::fmt(input, f)).value(input)
        }).finish()
    }
}

impl<const I: usize> TryFrom<Vec<Rc<Input, Bump>>> for Inputs<I> {
    type Error = <Box<[Rc<Input, Bump>; I]> as TryFrom<Vec<Rc<Input, Bump>>>>::Error;

    fn try_from(value: Vec<Rc<Input, Bump>>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

