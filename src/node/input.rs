use crate::pop::Pop;
use super::Node;
use core::{array, pin::Pin};

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    innov: usize,
    bias: f32,
}

impl Input {
    pub fn new(innov: usize) -> Self {
        Pop::next_node_innov();
        Self { innov, bias: f32::default() }
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

// a heap allocated array of inputs that guarantees that inputs do not move
pub struct Inputs<const I: usize>(Pin<Box<[Input; I]>>);

impl<const I: usize> Inputs<I> {
    pub fn new() -> Self {
        Self(Box::pin(array::from_fn::<_, I, _>(|innov| Input::new(innov))))
    }

    pub fn get(&self, index: usize) -> Option<Pin<&Input>> {
        Some(unsafe { Pin::new_unchecked(self.0.get(index)?) })
    }

    pub fn iter(&self) -> Iter<I> {
        let inputs = unsafe { Pin::new_unchecked(self.0.as_slice()) };
        Iter { inputs: self, index: 0 }
    }
}

pub struct Iter<'a, const I: usize> {
    inputs: &'a Inputs<I>,
    index: usize,
}

impl<'a, const I: usize> Iterator for Iter<'a, I> {
    type Item = Pin<&'a Input>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inputs.get(self.index);
        self.index += 1;
        next
    }
}
