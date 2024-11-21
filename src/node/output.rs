use crate::{pop::Pop, node::*, node::Accum};
use core::{cell::Cell, cmp, fmt, hash, mem::ManuallyDrop, pin::Pin};
use std::array;
use hashbrown::HashMap;

pub type Output = ManuallyDrop<Pin<Box<Inner>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Inner {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Inner {
    pub fn new(innov: usize) -> Self {
        Pop::next_node_innov();
        Self {
            layer: 1.into(),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov,
        }
    }

    pub fn idx<const I: usize>(&self) -> usize {
        self.innov - I
    }

    fn from_inner(&self) -> Output {
        ManuallyDrop::new(Box::into_pin(unsafe { Box::from_raw(self as *const _ as *mut _) }))
    }

    pub fn eval(&self, map: &mut HashMap<Head, Accum>) -> f32 {
        let input = map.get_mut(&Head::from(self.from_inner())).unwrap().eval(self.aggregator);
        self.activate(self.bias() + (self.response() * input))
    }
}

impl Node for Inner {
    fn layer(&self) -> usize { self.layer.get() }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { self.layer.update(|current| cmp::max(current, layer)); }
    fn activate(&self, x: f32) -> f32 { self.activation.get()(x)}
    fn response(&self) -> f32 { self.response }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { self.aggregator }
}

impl Eq for Inner {}

impl hash::Hash for Inner {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.layer.get().hash(state);
        self.response.to_bits().hash(state);
        self.bias.to_bits().hash(state);
        self.innov.hash(state);
    }
}

// a heap allocated array of outputs that guarantees that outputs do not move
pub struct Outputs<const I: usize, const O: usize>(Pin<Box<[Inner; O]>>);

impl<const I: usize, const O: usize> Outputs<I, O> {
    pub fn new() -> Self {
        Self(Box::pin(array::from_fn::<_, O, _>(|innov| Inner::new(I + innov))))
    }

    pub fn get(&self, index: usize) -> Option<Output> {
        Some(ManuallyDrop::new(Box::into_pin(unsafe { Box::from_raw(self.0.get(index)? as *const _ as *mut _) })))
    }

    pub fn eval_nth(&self, n: usize, map: &mut HashMap<Head, Accum>) -> f32 {
        self.0.get(n).unwrap().eval(map)
    }

    pub fn iter(&self) -> Iter<I, O> {
        Iter { outputs: self, index: 0 }
    }
}

impl<const I: usize, const O: usize> fmt::Debug for Outputs<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(&mut f.debug_map(), |f, ref output| {
            f.key_with(|f| fmt::Pointer::fmt(output, f)).value(output)
        }).finish()
    }
}

pub struct Iter<'a, const I: usize, const O: usize> {
    outputs: &'a Outputs<I, O>,
    index: usize,
}

impl<'a, const I: usize, const O: usize> Iterator for Iter<'a, I, O> {
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.outputs.get(self.index);
        self.index += 1;
        next
    }
}
