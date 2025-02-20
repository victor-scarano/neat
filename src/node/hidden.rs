extern crate alloc;
use crate::{arena::Arena, edge::Edge, node::{Accum, *}, pop::Pop};
use core::{cell::Cell, cmp, fmt, hash::{Hash, Hasher}, ptr, slice};
use hashbrown::HashMap;

#[derive(Clone)]
pub struct Hidden {
    innov: usize,
    layer: Cell<usize>,
    bias: f32,
    resp: f32,
    activ: Cell<fn(f32) -> f32>,
    aggreg: fn(&[f32]) -> f32,
}

impl Hidden {
    pub fn downgrade(&self) -> RawHidden {
        RawHidden::from(self)
    }

    pub fn from_edge(edge: &Edge) -> Self {
        let curr_level = edge.tail.layer();
        edge.head.update_layer(curr_level + 1);

        Self {
            layer: Cell::new(curr_level),
            activ: Cell::new(|x| x),
            aggreg: |values| values.iter().sum::<f32>() / (values.len() as f32),
            resp: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
        }
    }

    pub fn eval<'a>(&'a self, weight: f32, map: &mut HashMap<&'a Head<'a>, Accum>) -> f32 {
        let input = map.get_mut(&Head::from(self)).unwrap().eval(self.aggreg);
        weight * self.activate(self.bias() + (self.response() * input))
    }
}

impl fmt::Debug for Hidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f
            .debug_struct("Hidden")
            .field("innov", &self.innov)
            .field("layer", &self.layer.get())
            .field("bias", &self.bias)
            // .field("resp", &self.resp)
            // .field("activ", &self.activ.get())
            // .field("aggreg", &self.aggreg)
            .finish_non_exhaustive()
    }
}

impl Eq for Hidden {}

impl Node for Hidden {
    fn layer(&self) -> usize { self.layer.get() }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { self.layer.update(|current| cmp::max(current, layer)); }
    fn activate(&self, x: f32) -> f32 { self.activ.get()(x) }
    fn response(&self) -> f32 { self.resp }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { self.aggreg }
}

impl Hash for Hidden {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.resp.to_ne_bytes().hash(state);
        self.bias.to_ne_bytes().hash(state);
        self.innov.hash(state);
    }
}

impl PartialEq for Hidden {
    fn eq(&self, other: &Self) -> bool {
        self.resp == other.resp && self.bias == other.bias && self.innov == other.innov
    }
}

#[derive(Debug, Default)]
pub struct Hiddens {
    arena: Arena<Hidden>,
    len: usize,
}

impl Hiddens {
    fn insert<'a>(&mut self, edge: &Edge) -> &'a Hidden {
        self.len += 1;
        self.arena.push(Hidden::from_edge(edge))
    }

    pub fn split_edge<'a>(&mut self, edge: &Edge<'a>) -> (Edge<'a>, Edge<'a>) {
        edge.enabled.set(false);
        let middle = self.insert(edge);
        let first = Edge::new(edge.tail.clone(), middle);
        let last = Edge::new(middle, edge.head.clone());
        (first, last)
    }

    pub fn iter(&self) -> slice::Iter<'_, Hidden> {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Eq)]
pub struct RawHidden(*const Hidden);

impl RawHidden {
    pub fn upgrade<'a>(&self) -> &'a Hidden {
         unsafe { &*self.0 }
    }
}

impl From<&Hidden> for RawHidden {
    fn from(value: &Hidden) -> Self {
        Self(value as *const _)
    }
}

impl From<&mut Hidden> for RawHidden {
    fn from(value: &mut Hidden) -> Self {
        Self(value as *const _)
    }
}

impl Hash for RawHidden {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.upgrade().hash(state);
    }
}

impl PartialEq for RawHidden {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0, other.0)
    }
}

impl fmt::Pointer for RawHidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

