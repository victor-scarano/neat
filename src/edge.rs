extern crate alloc;
use crate::{node::*, pop::Pop};
use core::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    convert::Into,
    fmt,
    hash,
    iter::{IntoIterator, Map},
    mem::{self, MaybeUninit},
    slice
};
use alloc::collections::btree_set::{self, BTreeSet};
use bumpalo::Bump;
use hashbrown::{DefaultHashBuilder, hash_set::{Difference, HashSet, Intersection, SymmetricDifference}};
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Edge {
    tail: RawTail,
    head: RawHead,
    pub weight: f32,
    pub enabled: Cell<bool>,
    pub layer: usize,
    pub innov: usize,
}

impl Edge {
    pub fn new<'a>(tail: impl Into<Tail<'a>>, head: impl Into<Head<'a>>) -> Self {
        let tail: Tail = tail.into();
        let head: Head = head.into();

        // this assert ensures that we arent creating an edge from two of the
        // same node. however, this assertion is incorrectly failing when
        // comparing two different nodes, leading me to believe that the bug is
        // rooted in their partialeq impls... although, it could also have
        // something to do with the conversion between raw and "cooked" nodes
        // within the impls.
        assert_ne!(tail, head);

        head.update_layer(tail.layer() + 1);

        Self {
            innov: Pop::next_edge_innov(&tail, &head),
            layer: tail.layer(),
            enabled: Cell::new(true),
            weight: 1.0,
            tail: tail.downgrade(),
            head: head.downgrade(),
        }
    }

    pub fn tail(&self) -> Tail {
        self.tail.upgrade()
    }

    pub fn head(&self) -> Head {
        self.head.upgrade()
    }
}

impl Eq for Edge {}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f
            .debug_struct("Edge")
            .field_with("tail", |f| fmt::Pointer::fmt(&self.tail, f))
            .field_with("head", |f| fmt::Pointer::fmt(&self.head, f))
            .field("weight", &self.weight)
            .field("enabled", &self.enabled.get())
            .field("layer", &self.layer)
            .field("innov", &self.innov)
            .finish()
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct RawEdge(*const Edge);

impl RawEdge {
    fn upgrade<'a>(&self) -> &'a Edge {
        unsafe { &*self.0 }
    }
}

impl From<&Edge> for RawEdge {
    fn from(value: &Edge) -> Self {
        Self(value)
    }
}

#[derive(Eq)]
pub struct RawOrdEdge(RawEdge);

impl RawOrdEdge {
    fn upgrade<'a>(&self) -> &'a Edge {
        self.0.upgrade()
    }
}

impl From<&Edge> for RawOrdEdge {
    fn from(value: &Edge) -> Self {
        Self(RawEdge(value))
    }
}

impl From<RawEdge> for RawOrdEdge {
    fn from(value: RawEdge) -> Self {
        Self(value)
    }
}

impl Ord for RawOrdEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.upgrade();
        let rhs = other.upgrade();
        lhs.layer.cmp(&rhs.layer).then(lhs.innov.cmp(&rhs.innov))
    }
}

impl PartialEq for RawOrdEdge {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade().layer == other.upgrade().layer
    }
}

impl PartialOrd for RawOrdEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq)]
pub struct RawHashEdge(RawEdge);

impl RawHashEdge {
    fn upgrade(&self) -> &Edge {
        self.0.upgrade()
    }
}

impl From<&Edge> for RawHashEdge {
    fn from(value: &Edge) -> Self {
        Self(RawEdge(value))
    }
}

impl From<RawEdge> for RawHashEdge {
    fn from(value: RawEdge) -> Self {
        Self(value)
    }
}

impl hash::Hash for RawHashEdge {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.upgrade().innov.hash(state);
    }
}

impl PartialEq for RawHashEdge {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade().innov == other.upgrade().innov
    }
}

pub struct Edges<const CHUNK_LEN: usize = 32> {
    bump: RefCell<Bump>,
    btree: BTreeSet<RawOrdEdge>,
    hash: HashSet<RawHashEdge>,
}

impl<const CHUNK_LEN: usize> Edges<CHUNK_LEN> {
    pub fn new() -> Self {
        assert_ne!(CHUNK_LEN, 0);
        let bump = Bump::new();
        bump.set_allocation_limit(Some(CHUNK_LEN * size_of::<Edge>()));

        Self {
            bump: RefCell::new(bump),
            btree: BTreeSet::new(),
            hash: HashSet::new(),
        }
    }

    pub fn get(&self, edge: &Edge) -> Option<&Edge> {
        let edge = RawHashEdge::from(edge);
        self.hash.get(&edge).map(RawHashEdge::upgrade)
    }

    pub fn insert(&mut self, edge: Edge) {
        let edge = RawEdge(self.bump.borrow().alloc(edge));
        assert!(self.btree.insert(RawOrdEdge(edge)), "edge has already been inserted");
        assert!(self.hash.insert(RawHashEdge(edge)), "edge has already been inserted");
    }

    pub fn iter(&self) -> Map<btree_set::Iter<'_, RawOrdEdge>, fn(&RawOrdEdge) -> &Edge> {
        self.btree.iter().map(RawOrdEdge::upgrade)
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(
            self.btree.len(),
            self.hash.len(),
            "the hashset and btreeset used in managing a genome's edges should always be the same"
        );
        self.hash.len()
    }

    pub fn innov_int<'a>(lhs: &'a Self, rhs: &'a Self) -> InnovInt<'a> {
        lhs.hash.intersection(&rhs.hash).map(RawHashEdge::upgrade)
    }

    pub fn innov_diff<'a, R: Rng>(lhs: &'a Self, rhs: &'a Self) -> InnovDiff<'a, R> {
        InnovDiff::Diff(lhs.hash.difference(&rhs.hash).map(RawHashEdge::upgrade))
    }

    pub fn innov_sym_diff<'a, R: Rng>(lhs: &'a Self, rhs: &'a Self, rng: &mut R) -> InnovDiff<'a, R> {
        // let a = InnovDiff::SymDiff(lhs.hash.symmetric_difference(&rhs.hash));
            // .filter_map(|edge| rng.gen::<bool>().then_some(edge.upgrade())));
        todo!()
    }
}

// this clone impl hasnt been tested yet so idek if it works lmao
// the thing thats making me have my doubts is whether or not allocating the
// slice to the bump and extending the slice to the collections necessarily
// means that they are "tied" together
impl<const CHUNK_LEN: usize> Clone for Edges<CHUNK_LEN> {
    fn clone(&self) -> Self {
        let mut bump = self.bump.borrow_mut();
        let mut chunks = bump.iter_allocated_chunks();

        let bump = Bump::new();
        let mut btree = BTreeSet::new();
        let mut hash = HashSet::new();

        if let Some(chunk) = chunks.next() {
            let ptr = MaybeUninit::slice_as_ptr(chunk) as *const Edge;

            let len = match self.len() % CHUNK_LEN {
                0 => self.len(),
                non_zero => non_zero,
            };

            let chunk = unsafe { slice::from_raw_parts(ptr, len) };

            let edges = bump.alloc_slice_clone(chunk).iter();
            btree.extend(edges.clone().map(RawOrdEdge::from));
            hash.extend(edges.map(RawHashEdge::from));
        }

        for chunk in chunks {
            let chunk: &[Edge] = unsafe { mem::transmute(chunk) };

            let edges = bump.alloc_slice_clone(chunk).iter();
            btree.extend(edges.clone().map(RawOrdEdge::from));
            hash.extend(edges.map(RawHashEdge::from));
        }

        Self { bump: RefCell::new(bump), btree, hash }
    }
}

impl fmt::Debug for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a> Extend<&'a Edge> for Edges {
    fn extend<T: IntoIterator<Item = &'a Edge>>(&mut self, iter: T) {
        let bump = self.bump.borrow();
        let edges = iter.into_iter().cloned().map(|edge| RawEdge(bump.alloc(edge)));

        // cant call clone because T::Iter is not Clone
        // self.btree.extend(edges.clone().map(RawOrdEdge::from));
        // self.hash.extend(edges.map(RawHashEdge::from));
        
        for edge in edges {
            self.btree.insert(RawOrdEdge::from(edge));
            self.hash.insert(RawHashEdge::from(edge));
        }
    }
}

type InnovInt<'a> = Map<Intersection<'a, RawHashEdge, DefaultHashBuilder>, fn(&RawHashEdge) -> &Edge>;
type Diff<'a> = Map<Difference<'a, RawHashEdge, DefaultHashBuilder>, fn(&RawHashEdge) -> &Edge>;
type SymDiff<'a> = Map<SymmetricDifference<'a, RawHashEdge, DefaultHashBuilder>, fn(&RawHashEdge) -> &Edge>;

pub enum InnovDiff<'a, R: Rng> {
    Diff(Diff<'a>),
    SymDiff { iter: SymDiff<'a>, rng: &'a mut R },
}

impl<'a, R: Rng> Iterator for InnovDiff<'a, R> {
    type Item = &'a Edge;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Diff(diff) => diff.next(),
            Self::SymDiff { iter, rng } => iter.find(|_| rng.gen())
        }
    }
}
