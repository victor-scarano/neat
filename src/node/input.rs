use crate::{node::Node, pop::Pop};
use core::{array, fmt, marker::PhantomPinned, pin::Pin};

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    innov: usize,
    bias: f32,
    _pinned: PhantomPinned,
}

impl Input {
    pub fn new(innov: usize) -> Self {
        Pop::next_node_innov();
        Self { innov, bias: 0.0, _pinned: PhantomPinned }
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

pub struct InputArena<const I: usize>(Pin<Box<[Input; I]>>);

impl<const I: usize> InputArena<I> {
    pub fn new() -> Self {
        Self(Box::pin(array::from_fn::<_, I, _>(|innov| Input::new(innov))))
    }

    pub fn get(&self, index: usize) -> Option<Pin<&Input>> {
        Some(unsafe { Pin::new_unchecked(self.0.get(index)?) })
    }

    pub fn iter(&self) -> Iter<I> {
        Iter { inputs: self, index: 0 }
    }
}

impl<const I: usize> fmt::Debug for InputArena<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(&mut f.debug_map(), |f, ref input| {
            f.key_with(|f| fmt::Pointer::fmt(input, f)).value(input)
        }).finish()
    }
}

pub struct Iter<'genome, const I: usize> {
    inputs: &'genome InputArena<I>,
    index: usize,
}

impl<'genome, const I: usize> Iterator for Iter<'genome, I> {
    type Item = Pin<&'genome Input>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inputs.get(self.index);
        self.index += 1;
        next
    }
}

