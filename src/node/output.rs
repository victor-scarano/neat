use crate::{pop::Pop, node::*, node::Accum};
use core::{array, cell::Cell, cmp, fmt, hash, marker::PhantomPinned, pin::Pin};
use hashbrown::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
    _pinned: PhantomPinned,
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
            _pinned: PhantomPinned,
        }
    }

    pub fn idx<const I: usize>(&self) -> usize {
        self.innov - I
    }

    pub fn eval<'a>(self: Pin<&'a Self>, map: &mut HashMap<Head<'a>, Accum>) -> f32 {
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

impl hash::Hash for Output {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.layer.get().hash(state);
        self.response.to_bits().hash(state);
        self.bias.to_bits().hash(state);
        self.innov.hash(state);
    }
}

// a heap allocated array of outputs that guarantees that outputs do not move
pub struct OutputArena<const I: usize, const O: usize>(Pin<Box<[Output; O]>>);

impl<const I: usize, const O: usize> OutputArena<I, O> {
    pub fn new() -> Self {
        Self(Box::pin(array::from_fn::<_, O, _>(|innov| Output::new::<I>(innov))))
    }

    pub fn get(&self, index: usize) -> Option<Pin<&Output>> {
        Some(unsafe { Pin::new_unchecked(self.0.get(index)?) })
    }

    pub fn eval_nth<'a>(&'a self, n: usize, map: &mut HashMap<Head<'a>, Accum>) -> f32 {
        self.get(n).unwrap().eval(map)
    }

    pub fn iter(&self) -> Iter<I, O> {
        Iter { outputs: self, index: 0 }
    }
}

impl<const I: usize, const O: usize> fmt::Debug for OutputArena<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(&mut f.debug_map(), |f, ref output| {
            f.key_with(|f| fmt::Pointer::fmt(output, f)).value(output)
        }).finish()
    }
}

pub struct Iter<'genome, const I: usize, const O: usize> {
    outputs: &'genome OutputArena<I, O>,
    index: usize,
}

impl<'genome, const I: usize, const O: usize> Iterator for Iter<'genome, I, O> {
    type Item = Pin<&'genome Output>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.outputs.get(self.index);
        self.index += 1;
        next
    }
}
