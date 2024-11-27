extern crate alloc;
use crate::{node::Node, pop::Pop};
use core::{array, fmt, slice};
use alloc::rc::Rc;
use bumpalo::{boxed::Box, Bump};

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    innov: usize,
    bias: f32,
}

impl Input {
    pub fn new_in(innov: usize, bump: &Bump) -> Rc<Self, &Bump> {
        Pop::next_node_innov();
        Rc::new_in(Self { innov, bias: 0.0 }, bump)
    }

    // we can use self.innov as the idx for any input node
    pub fn idx(&self) -> usize {
        self.innov
    }

    pub fn eval<const I: usize>(&self, weight: f32, inputs: [f32; I]) -> f32 {
        weight * (self.bias() + inputs[self.idx()])
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

pub struct Inputs<'genome, const I: usize>(Box<'genome, [Rc<Input, &'genome Bump>; I]>);

impl<'genome, const I: usize> Inputs<'genome, I> {
    pub fn new_in(bump: &'genome Bump) -> Self {
        Self(Box::new_in(array::from_fn::<_, I, _>(|innov| Input::new_in(innov, bump)), bump))
    }

    pub fn get(&self, index: usize) -> Option<Rc<Input, &'genome Bump>> {
        self.0.get(index).cloned()
    }

    pub fn iter(&self) -> slice::Iter<Rc<Input, &'genome Bump>> {
        self.0.iter()
    }
}

impl<const I: usize> fmt::Debug for Inputs<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(&mut f.debug_map(), |f, ref input| {
            f.key_with(|f| fmt::Pointer::fmt(input, f)).value(input)
        }).finish()
    }
}

