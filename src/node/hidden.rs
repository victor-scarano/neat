extern crate alloc;
use crate::{edge::Edge, pop::Pop, node::*, node::Accum};
use core::{
    cell::{Cell, RefCell},
    cmp,
    fmt,
    hash::{Hash, Hasher},
    mem::{self, MaybeUninit},
    ptr::NonNull,
    slice
};
use alloc::{rc::Rc, vec::Vec};
use bumpalo::{Bump, ChunkIter};
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
    pub fn from_edge(edge: &Edge) -> Self {
        let curr_level = edge.tail().layer();
        edge.head().update_layer(curr_level + 1);

        Self {
            layer: Cell::new(curr_level),
            activ: Cell::new(|x| x),
            aggreg: |values| values.iter().sum::<f32>() / (values.len() as f32),
            resp: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
        }
    }

    pub fn eval<'a>(&'a self, weight: f32, map: &mut HashMap<Head<'a>, Accum>) -> f32 {
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

pub struct Hiddens {
    bump: RefCell<Bump>,
    len: usize,
}

impl Hiddens {
    pub fn new() -> Self {
        let bump = Bump::new();
        // is it necessary to set the allocation limit here?
        Self { bump: RefCell::new(bump), len: 0 }
    }

    fn insert(&mut self, edge: &Edge) -> RawHidden {
        let new = self.bump.borrow().alloc(Hidden::from_edge(edge)) as *const _;
        self.len += 1;
        RawHidden(new)
    }

    pub fn split_edge(&mut self, edge: &Edge) -> (Edge, Edge) {
        let middle = self.insert(edge);
        let first = Edge::new(edge.tail(), unsafe { middle.upgrade() });
        let last = Edge::new(unsafe { middle.upgrade() }, edge.head());
        (first, last)
    }

    pub fn iter(&self) -> Iter<'_> {
        let mut bump = self.bump.borrow_mut();
        let mut chunks = bump.iter_allocated_chunks();
        let mut hiddens = Vec::new();

        if let Some(chunk) = chunks.next() {
            let ptr = MaybeUninit::slice_as_ptr(chunk) as *const Hidden;
            let slice = unsafe { slice::from_raw_parts(ptr, self.len) };
            hiddens.extend(slice);
        }

        for chunk in chunks {
            let slice: &[Hidden] = unsafe { mem::transmute(chunk) };
            hiddens.extend(slice);
        }

        Iter(hiddens)
    }
}

impl fmt::Debug for Hiddens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // note that this debug impl does not reflect the fact that this struct
        // internally manages a bump allocator or a length.
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<'a>(Vec<&'a Hidden>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Hidden;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl fmt::Debug for Iter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: check if Vec::as_slice makes any difference in debug output.
        // the goal is for the debug output to match slice::Iter's debug output,
        // not only to be consistent with the std lib, but also to be consistent
        // with the other debug outputs of the other node collections.
        f.debug_tuple("Iter").field(&self.0.as_slice()).finish()
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

