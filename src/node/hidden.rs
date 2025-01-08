extern crate alloc;
use crate::{edge::Edge, pop::Pop, node::*, node::Accum};
use core::{cell::Cell, cmp, hash::{Hash, Hasher}};
use alloc::rc::Rc;
use bumpalo::Bump;
use hashbrown::HashMap;

#[derive(Clone, Debug)]
pub struct Hidden {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Hidden {
    pub fn from_edge(edge: &Edge) -> Self {
        let curr_level = edge.tail.layer();
        edge.head.update_layer(curr_level + 1);

        Self {
            layer: Cell::new(curr_level),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
        }
    }

    pub fn eval(&self, weight: f32, map: &mut HashMap<Head, Accum>) -> f32 {
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

impl Hash for Hidden {
    fn hash<H: Hasher>(&self, state: &mut H) {
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

pub struct Hiddens {
    bump: Bump,
}

impl Hiddens {
    pub fn new() -> Self {
        Self { bump: Bump::new() }
    }

    pub fn from_edge(&self, edge: &Edge) -> RawHidden {
        RawHidden(self.bump.alloc(Hidden::from_edge(edge)))
    }
}

// should partial eq check for ptr eq or value eq?
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RawHidden(*const Hidden);

impl AsRef<Hidden> for RawHidden {
    fn as_ref(&self) -> &Hidden {
        unsafe { &*self.0 }
    }
}

impl From<&Hidden> for RawHidden {
    fn from(value: &Hidden) -> Self {
        Self(value as *const _)
    }
}

impl Hash for RawHidden {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

