use crate::{conn::Conn, pop::Pop, node::*, node::accum::Accum};
use core::{cell::Cell, cmp, hash, iter};
use hashbrown::{HashMap, hash_set::{self, HashSet}};

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
    pub fn new(conn: &Conn) -> Self {
        let curr_level = conn.tail.layer();
        conn.head.update_layer(curr_level + 1);

        Self {
            layer: Cell::new(curr_level),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
        }
    }

    pub fn eval(self: &Self, weight: f32, map: &mut HashMap<Head, Accum>) -> f32 {
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
        self.response.to_le_bytes().hash(state);
        self.bias.to_le_bytes().hash(state);
        self.innov.hash(state);
    }
}

impl PartialEq for Hidden {
    fn eq(&self, other: &Self) -> bool {
        self.response == other.response && self.bias == other.bias && self.innov == other.innov
    }
}

pub struct Hiddens(HashSet<Pin<Box<Hidden>>>);

impl Hiddens {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = Pin<&Hidden>> {
        self.0.iter().map(<Pin<Box<Hidden>>>::as_ref)
    }
}
