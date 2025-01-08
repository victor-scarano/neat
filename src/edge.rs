extern crate alloc;
use crate::{genome::Genome, node::*, pop::Pop};
use core::{cell::Cell, cmp::Ordering, convert::Into, fmt, hash, iter, mem};
use alloc::{collections::BTreeSet, rc::*};
use bumpalo::Bump;
use hashbrown::HashSet;
use rand::{seq::IteratorRandom, Rng};

#[derive(Clone)]
pub struct Edge {
    tail: RawTail,
    head: RawHead,
    pub weight: f32,
    pub enabled: Cell<bool>,
    pub layer: usize,
    pub innov: usize,
}

impl Edge {
    pub fn new(tail: impl Into<Tail>, head: impl Into<Head>) -> Self {
        let tail = tail.into();
        let head = head.into();
        assert_ne!(tail, head);

        head.update_layer(tail.layer() + 1);

        Self {
            innov: Pop::next_edge_innov(&tail, &head),
            layer: tail.layer(),
            enabled: Cell::new(true),
            weight: 1.0,
            tail: tail.into(),
            head: head.into(),
        }
    }

    pub fn tail(&self) -> Tail {
        self.tail.into()
    }

    pub fn head(&self) -> Head {
        self.head.into()
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

impl hash::Hash for Edge {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.innov.hash(state);
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.enabled.get()
        //    .cmp(&other.enabled.get())
        //    .reverse()
        self.layer.cmp(&other.layer).then(self.innov.cmp(&other.innov))
    }
}

// used to be equal if innovations were equal, but needs to reflect ord impl
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq() && self.innov == other.innov
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq)]
struct RawEdge(*const Edge);

impl RawEdge {
    pub unsafe fn upgrade<'a>(&self) -> &'a Edge {
        unsafe { &*self.0 }
    }
}

impl hash::Hash for RawEdge {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let inner = unsafe { self.upgrade() };
        inner.hash(state);
    }
}

impl Ord for RawEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = unsafe { self.upgrade() };
        let rhs = unsafe { other.upgrade() };
        lhs.cmp(rhs)
    }
}

impl PartialEq for RawEdge {
    fn eq(&self, other: &Self) -> bool {
        let lhs = unsafe { self.upgrade() };
        let rhs = unsafe { other.upgrade() };
        lhs.eq(rhs)
    }
}

impl PartialOrd for RawEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Edges {
    bump: Bump,
    btree: BTreeSet<RawEdge>,
    hash: HashSet<RawEdge>,
}

impl Edges {
    pub fn new() -> Self {
        Self {
            bump: Bump::new(),
            btree: BTreeSet::new(),
            hash: HashSet::new(),
        }
    }

    pub fn get(&self, edge: &Edge) -> Option<&Edge> {
        let edge = RawEdge(edge as *const _);
        self.hash.get(&edge).map(|edge| unsafe { edge.upgrade() })
    }

    pub fn insert(&mut self, edge: Edge) {
        let edge = RawEdge(self.bump.alloc(edge));
        assert!(self.btree.insert(edge), "edge has already been inserted");
        assert!(self.hash.insert(edge), "edge has already been inserted");
    }

    pub fn iter_ordered(&self) -> impl Iterator<Item = &Edge> {
        self.btree.iter().map(|edge| unsafe { edge.upgrade() })
    }

    pub fn iter_unordered(&self) -> impl Iterator<Item = &Edge> {
        self.hash.iter().map(|edge| unsafe { edge.upgrade() })
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.btree.len(), self.hash.len());
        self.hash.len() // is one len method faster than the other?
    }
}

impl iter::Extend<RawEdge> for Edges {
    fn extend<T: IntoIterator<Item = RawEdge>>(&mut self, iter: T) {
        let mut iter = iter.into_iter();
        self.btree.extend(&mut iter);
        self.hash.extend(&mut iter);
    }
}

impl fmt::Debug for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter_ordered()).finish()
    }
}

