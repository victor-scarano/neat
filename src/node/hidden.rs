use crate::{edge::Edge, pop::Pop, node::*, node::Accum};
use core::{cell::Cell, cmp, fmt, hash, marker::PhantomPinned, mem::ManuallyDrop, pin::Pin};
use hashbrown::{HashMap, HashSet};

pub type Hidden = ManuallyDrop<Pin<Box<Inner>>>;

#[derive(Clone, Debug)]
struct Inner {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
    _pinned: PhantomPinned
}

impl Inner {
    fn from_edge(edge: &Edge) -> *mut Self {
        let curr_level = edge.tail.layer();
        edge.head.update_layer(curr_level + 1);

        Box::into_raw(Box::new(Self {
            layer: Cell::new(curr_level),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
            _pinned: PhantomPinned
        }))
    }

    fn from_inner(&self) -> Hidden {
        ManuallyDrop::new(Box::into_pin(unsafe { Box::from_raw(self as *const _ as *mut _) }))
    }

    fn from_raw_inner(inner: *mut Self) -> Hidden {
        ManuallyDrop::new(Box::into_pin(unsafe { Box::from_raw(inner) }))
    }

    pub fn eval(&self, weight: f32, map: &mut HashMap<Head, Accum>) -> f32 {
        let input = map.get_mut(&Head::from(self.from_inner())).unwrap().eval(self.aggregator);
        weight * self.activate(self.bias() + (self.response() * input))
    }
}

impl Node for Inner {
    fn layer(&self) -> usize { self.layer.get() }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { self.layer.update(|current| cmp::max(current, layer)); }
    fn activate(&self, x: f32) -> f32 { self.activation.get()(x) }
    fn response(&self) -> f32 { self.response }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { self.aggregator }
}

impl Eq for Inner {}

impl hash::Hash for Inner {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.response.to_ne_bytes().hash(state);
        self.bias.to_ne_bytes().hash(state);
        self.innov.hash(state);
    }
}

impl PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        self.response == other.response && self.bias == other.bias && self.innov == other.innov
    }
}

pub struct Hiddens(HashSet<*mut Inner>);

impl Hiddens {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    // must always insert, but cant check to make sure it inserted
    // we want insert_then_get functionality
    pub fn insert_from_edge_and_get(&mut self, edge: &Edge) -> Hidden {
        Inner::from_raw_inner(*self.0.get_or_insert(Inner::from_edge(edge)))
    }

    pub fn iter(&self) -> impl Iterator<Item = Hidden> + '_ {
        self.0.iter().copied().map(Inner::from_raw_inner)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Drop for Hiddens {
    fn drop(&mut self) {
        for raw in self.0.iter() {
            drop(unsafe { Box::from_raw(*raw) });
        }
    }
}

impl fmt::Debug for Hiddens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(&mut f.debug_map(), |f, hidden| {
            f.key_with(|f| fmt::Pointer::fmt(hidden, f)).value(hidden)
        }).finish()
    }
}

