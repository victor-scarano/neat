use crate::{node::Node, pop::Pop};
use core::{array, fmt, mem::ManuallyDrop, pin::Pin};

pub type Input = ManuallyDrop<Pin<Box<Inner>>>;

#[derive(Clone, Debug, PartialEq)]
struct Inner {
    innov: usize,
    bias: f32,
}

impl Inner {
    pub fn new(innov: usize) -> Self {
        Pop::next_node_innov();
        Self { innov, bias: 0.0 }
    }

    // we can use self.innov as the idx for any input node
    pub fn idx(&self) -> usize {
        self.innov
    }

    pub fn eval<const I: usize>(&self, weight: f32, inputs: [f32; I]) -> f32 {
        weight * (self.bias() + inputs[self.idx()])
    }
}

impl Node for Inner {
    fn layer(&self) -> usize { 0 }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { panic!(); }
    fn activate(&self, x: f32) -> f32 { panic!(); }
    fn response(&self) -> f32 { panic!(); }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { panic!(); }
}

pub struct Inputs<const I: usize>(Pin<Box<[Inner; I]>>);

impl<const I: usize> Inputs<I> {
    pub fn new() -> Self {
        Self(Box::pin(array::from_fn::<_, I, _>(|innov| Inner::new(innov))))
    }

    pub fn get(&self, index: usize) -> Option<Input> {
        Some(ManuallyDrop::new(Box::into_pin(unsafe { Box::from_raw(self.0.get(index)? as *const _ as *mut _) })))
    }

    pub fn iter(&self) -> Iter<I> {
        Iter { inputs: self, index: 0 }
    }
}

impl<const I: usize> fmt::Debug for Inputs<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(&mut f.debug_map(), |f, ref input| {
            f.key_with(|f| fmt::Pointer::fmt(input, f)).value(input)
        }).finish()
    }
}

pub struct Iter<'a, const I: usize> {
    inputs: &'a Inputs<I>,
    index: usize,
}

impl<'a, const I: usize> Iterator for Iter<'a, I> {
    type Item = Input;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inputs.get(self.index);
        self.index += 1;
        next
    }
}

