extern crate alloc;
use crate::{edge::Edge, pop::Pop, node::*, node::Accum};
use core::{cell::Cell, cmp, fmt, hash::{Hash, Hasher}, mem::{self, MaybeUninit}, ptr::NonNull, slice};
use alloc::{rc::Rc, vec::Vec};
use bumpalo::{Bump, ChunkIter};
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
        let curr_level = edge.tail().layer();
        edge.head().update_layer(curr_level + 1);

        Self {
            layer: Cell::new(curr_level),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
        }
    }

    pub fn eval<'a>(&'a self, weight: f32, map: &mut HashMap<Head<'a>, Accum>) -> f32 {
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

#[derive(Debug)]
pub struct Hiddens<const N: usize = 32> {
    bump: Bump,
    len: usize,
}

impl<const N: usize> Hiddens<N> {
    pub fn new() -> Self {
        let bump = Bump::new();
        bump.set_allocation_limit(Some(N * size_of::<Hidden>()));
        Self { bump, len: 0 }
    }

    fn insert(&mut self, edge: &Edge) -> RawHidden {
        let new = self.bump.alloc(Hidden::from_edge(edge));
        self.len += 1;
        RawHidden(new)
    }

    pub fn split_edge(&mut self, edge: &Edge) -> (Edge, Edge) {
        let middle = self.insert(edge);
        let first = Edge::new(edge.tail(), unsafe { middle.upgrade() });
        let last = Edge::new(unsafe { middle.upgrade() }, edge.head());
        (first, last)
    }

    pub fn iter(&mut self) -> Iter<'_, N> {
        Iter::new(&mut self.bump, self.len)
    }
}

pub struct Iter<'a, const N: usize> {
    chunks: ChunkIter<'a>,
    curr: slice::Iter<'a, Hidden>,
}

impl<'a, const N: usize> Iter<'a, N> {
    fn new(bump: &'a mut Bump, len: usize) -> Self {
        let mut chunks = bump.iter_allocated_chunks();

        let curr = match chunks.next() {
            Some(chunk) => {
                let ptr = MaybeUninit::slice_as_ptr(chunk) as *const Hidden;
                let slice = unsafe { slice::from_raw_parts(ptr, len) };
                slice.iter()
            },
            None => slice::Iter::default(),
        };

        Self { chunks, curr }
    }
}

impl<'a, const N: usize> Iterator for Iter<'a, N> {
    type Item = &'a Hidden;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.next().or(self.chunks.next().and_then(|chunk| {
            let slice: &[Hidden] = unsafe { mem::transmute(chunk) };
            self.curr = slice.iter();
            self.curr.next()
        }))
    }
}

// should partial eq check for ptr eq or value eq?
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RawHidden(*const Hidden);

impl RawHidden {
    pub unsafe fn upgrade<'a>(&self) -> &'a Hidden {
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
        let inner = unsafe { self.upgrade() };
        inner.hash(state);
    }
}

impl fmt::Pointer for RawHidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

