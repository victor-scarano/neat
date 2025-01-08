extern crate alloc;
use crate::{pop::Pop, node::*, node::Accum};
use core::{array, cell::Cell, cmp, fmt, hash::{Hash, Hasher}, slice};
use alloc::{boxed::Box, rc::Rc, vec::Vec};
use hashbrown::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Output {
    pub fn new<const I: usize>(innov: usize) -> Self {
        Pop::next_node_innov();
        Self {
            layer: 1.into(),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: I - innov,
        }
    }

    pub fn index<const I: usize>(&self) -> usize {
        self.innov - I
    }

    pub fn eval<'a>(&'a self, map: &mut HashMap<Head<'a>, Accum>) -> f32 {
        let input = map.get_mut(&Head::from(self)).unwrap().eval(self.aggregator);
        self.activate(self.bias() + (self.response() * input))
    }
}

impl Node for Output {
    fn layer(&self) -> usize { self.layer.get() }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { self.layer.update(|current| cmp::max(current, layer)); }
    fn activate(&self, x: f32) -> f32 { self.activation.get()(x)}
    fn response(&self) -> f32 { self.response }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { self.aggregator }
}

impl Eq for Output {}

impl Hash for Output {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.layer.get().hash(state);
        self.response.to_bits().hash(state);
        self.bias.to_bits().hash(state);
        self.innov.hash(state);
    }
}

pub struct Outputs<const O: usize>(Box<[Output; O]>);

impl<const O: usize> Outputs<O> {
    pub fn new<const I: usize>() -> Self {
        Self(Box::new(array::from_fn::<_, O, _>(Output::new::<I>)))
    }

    pub fn get(&self, index: usize) -> Option<&Output> {
        self.0.get(index)
    }

    pub fn eval_nth<'a>(&'a self, n: usize, map: &mut HashMap<Head<'a>, Accum>) -> f32 {
        self.get(n).unwrap().eval(map)
    }

    pub fn iter(&self) -> slice::Iter<'_, Output> {
        self.0.iter()
    }
}

impl<const O: usize> fmt::Debug for Outputs<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(&mut f.debug_map(), |f, ref output| {
            f.key_with(|f| fmt::Pointer::fmt(output, f)).value(output)
        }).finish()
    }
}

impl<const O: usize> TryFrom<Vec<Output>> for Outputs<O> {
    type Error = <Box<[Output; O]> as TryFrom<Vec<Output>>>::Error;

    fn try_from(value: Vec<Output>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RawOutput(*const Output);

impl RawOutput {
    pub unsafe fn upgrade<'a>(&self) -> &'a Output {
        unsafe { &*self.0 }
    }
}

impl From<&Output> for RawOutput {
    fn from(value: &Output) -> Self {
        Self(value as *const _)
    }
}

impl Hash for RawOutput {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let inner = unsafe { self.upgrade() };
        inner.hash(state);
    }
}

impl fmt::Pointer for RawOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

