extern crate alloc;
use crate::{node::Node, pop::Pop};
use core::{array, fmt, hash, slice};
use alloc::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    innov: usize,
    bias: f32,
}

impl Input {
    pub fn new(innov: usize) -> Self {
        Pop::next_node_innov();
        Self { innov, bias: 0.0 }
    }

    // we can use self.innov as the idx for any input node
    pub fn index(&self) -> usize {
        self.innov
    }

    pub fn eval<const I: usize>(&self, weight: f32, inputs: [f32; I]) -> f32 {
        weight * (self.bias() + inputs[self.index()])
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

pub struct Inputs<const I: usize>(Box<[Input; I]>);

impl<const I: usize> Inputs<I> {
    pub fn new() -> Self {
        Self(Box::new(array::from_fn::<_, I, _>(|innov| Input::new(innov))))
    }

    pub fn get(&self, index: usize) -> Option<&Input> {
        self.0.get(index)
    }

    pub fn iter(&self) -> slice::Iter<'_, Input> {
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

impl<const I: usize> TryFrom<Vec<Input>> for Inputs<I> {
    type Error = <Box<[Input; I]> as TryFrom<Vec<Input>>>::Error;

    fn try_from(value: Vec<Input>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

// should partial eq check for ptr eq or value eq?
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RawInput(*const Input);

impl AsRef<Input> for RawInput {
    fn as_ref(&self) -> &Input {
        unsafe { &*self.0 }
    }
}

