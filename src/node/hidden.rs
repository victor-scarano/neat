use crate::{edge::Edge, pop::Pop, node::*, node::Accum};
use core::{cell::{Cell, UnsafeCell}, cmp, fmt, hash, marker::PhantomPinned, pin::Pin};
use hashbrown::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Hidden {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
    _pinned: PhantomPinned
}

impl Hidden {
    fn from_edge(edge: &Edge) -> Pin<Box<Self>> {
        let curr_level = edge.tail.layer();
        edge.head.update_layer(curr_level + 1);

        Box::pin(Self {
            layer: Cell::new(curr_level),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
            _pinned: PhantomPinned
        })
    }

    pub fn eval<'a>(self: Pin<&'a Self>, weight: f32, map: &mut HashMap<Head<'a>, Accum>) -> f32 {
        let input = map.get_mut(&Head::from(self)).unwrap().eval(self.aggregator);
        weight * self.activate(self.bias() + (self.response() * input))
    }
}

impl Node for Hidden {
    fn layer(&self) -> usize { self.layer.get() }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { self.layer.update(|current| cmp::max(current, layer)); }
    fn activate(&self, x: f32) -> f32 { self.activation.get()(x) }
    fn response(&self) -> f32 { self.response }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { self.aggregator }
}

impl Eq for Hidden {}

impl hash::Hash for Hidden {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.response.to_ne_bytes().hash(state);
        self.bias.to_ne_bytes().hash(state);
        self.innov.hash(state);
    }
}

impl PartialEq for Hidden {
    fn eq(&self, other: &Self) -> bool {
        self.response == other.response && self.bias == other.bias && self.innov == other.innov
    }
}

pub struct HiddenArena(UnsafeCell<HashSet<Pin<Box<Hidden>>>>);

impl HiddenArena {
    fn hash_set(&self) -> &HashSet<Pin<Box<Hidden>>> {
        unsafe { &*self.0.get() }
    }

    fn hash_set_mut(&self) -> &mut HashSet<Pin<Box<Hidden>>> {
        unsafe { &mut *self.0.get() }
    }

    pub fn new() -> Self {
        Self(UnsafeCell::new(HashSet::new()))
    }

    // must always insert, but cant check to make sure it inserted
    // we want insert_then_get functionality
    pub fn insert_from_edge_and_get(&self, edge: &Edge) -> Pin<&Hidden> {
        self.hash_set_mut().get_or_insert(Hidden::from_edge(edge)).as_ref()
    }

    pub fn iter(&self) -> impl Iterator<Item = Pin<&Hidden>> {
        self.hash_set().iter().map(<Pin<Box<Hidden>>>::as_ref)
    }

    pub fn len(&self) -> usize {
        self.hash_set().len()
    }
}

impl fmt::Debug for HiddenArena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.hash_set().iter().fold(&mut f.debug_map(), |f, hidden| {
            f.key_with(|f| fmt::Pointer::fmt(hidden, f)).value(hidden)
        }).finish()
    }
}

