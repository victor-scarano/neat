extern crate alloc;
use crate::{genome::Genome, node::*, pop::Pop};
use core::{cell::Cell, cmp::Ordering, convert::Into, fmt, hash, iter, mem};
use std::hash::{Hash, Hasher};
use alloc::{collections::BTreeSet, rc::*};
use bumpalo::Bump;
use hashbrown::HashSet;
use rand::{seq::IteratorRandom, Rng};

#[derive(Clone)]
pub struct Edge {
    pub tail: Tail,
    pub head: Head,
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
            tail,
            head,
        }
    }

    pub fn split(&self, hiddens: &Hiddens) -> (Edge, Edge) {
        let middle = hiddens.from_edge(self);
        let first = Edge::new(self.tail.clone(), middle.into());
        let last = Edge::new(middle.into(), self.head.clone());
        (first, last)
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

impl AsRef<Edge> for RawEdge {
    fn as_ref(&self) -> &Edge {
        unsafe { &*self.0 }
    }
}

impl Hash for RawEdge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl Ord for RawEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl PartialEq for RawEdge {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
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
        self.hash.get(&edge).map(RawEdge::as_ref)
    }

    pub fn insert(&mut self, edge: Edge) {
        let edge = RawEdge(self.bump.alloc(edge));
        assert!(self.btree.insert(edge));
        assert!(self.hash.insert(edge));
    }

    pub fn random_edges(&self, rng: &mut impl Rng) -> (&Edge, &Edge) {
        // returns two random nonequal edges
        assert!(self.len() >= 1);

        let mut edges = loop {
            let edges = self.iter_ordered().choose_multiple(rng, 2);

            if edges[0] != edges[1] {
                break edges;
            }
        };

        edges.sort_unstable();

        (edges[0], edges[1])
    }

    pub fn iter_ordered(&self) -> impl Iterator<Item = &Edge> {
        self.btree.iter().map(RawEdge::as_ref)
    }

    pub fn iter_unordered(&self) -> impl Iterator<Item = &Edge> {
        self.hash.iter().map(RawEdge::as_ref)
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.btree.len(), self.hash.len());
        self.hash.len() // is one len method faster than the other?
    }
}

impl iter::Extend<Edge> for Edges {
    fn extend<T: IntoIterator<Item = Edge>>(&mut self, iter: T) {
        let mut iter = iter.into_iter().map(Rc::new);
        self.btree.extend(&mut iter);
        self.hash.extend(&mut iter);
    }
}

impl fmt::Debug for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter_ordered()).finish()
    }
}
