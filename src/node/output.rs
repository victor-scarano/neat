extern crate alloc;
use crate::{pop::Pop, node::*, node::Accum};
use core::{cell::Cell, cmp, fmt, hash::{Hash, Hasher}, ptr};
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
    pub fn downgrade(&self) -> RawOutput {
        RawOutput::from(self)
    }

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

    pub fn eval<'a>(&'a self, map: &mut HashMap<&'a Head<'a>, Accum>) -> f32 {
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

#[derive(Clone, Copy, Debug, Eq)]
pub struct RawOutput(*const Output);

impl RawOutput {
    pub fn upgrade<'a>(&self) -> &'a Output {
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
        self.upgrade().hash(state);
    }
}

impl PartialEq for RawOutput {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0, other.0)
    }
}

impl fmt::Pointer for RawOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

