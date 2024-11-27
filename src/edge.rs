extern crate alloc;
use crate::{node::*, pop::Pop};
use core::{cell::Cell, cmp::Ordering, fmt, hash};
use alloc::{collections::BTreeSet, rc::*};
use hashbrown::HashSet;
use rand::{seq::IteratorRandom, Rng};

#[derive(Clone)]
pub struct Edge<'genome> {
    pub tail: Tail,
    pub head: Head<'genome>,
    pub weight: f32,
    pub enabled: Cell<bool>,
    pub layer: usize,
    pub innov: usize,
}

impl Edge<'_> {
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

    pub fn split(&self) -> (Edge, Edge) {
        let middle = Hidden::from_edge(self);
        let first = Edge::new(self.tail.clone(), middle.clone());
        let last = Edge::new(middle, self.head.clone());
        (first, last)
    }
}

impl Eq for Edge<'_> {}

impl fmt::Debug for Edge<'_> {
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

impl hash::Hash for Edge<'_> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.innov.hash(state);
    }
}

impl Ord for Edge<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.enabled.get()
        //    .cmp(&other.enabled.get())
        //    .reverse()
        self.layer.cmp(&other.layer).then(self.innov.cmp(&other.innov))
    }
}

// used to be equal if innovations were equal, but needs to reflect ord impl
impl PartialEq for Edge<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq() && self.innov == other.innov
    }
}

impl PartialOrd for Edge<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// TODO: Write custom Rc implementation to optimize for only two possible references so that the RcInner allocation
// isn't as large as it normally is
pub struct Edges<'genome> {
    btree_set: BTreeSet<Rc<Edge<'genome>>>,
    hash_set: HashSet<Rc<Edge<'genome>>>,
}

impl<'genome> Edges<'genome> {
    pub fn new() -> Self {
        Self {
            btree_set: BTreeSet::new(),
            hash_set: HashSet::new(),
        }
    }

    pub fn from(matching: Vec<&Edge<'genome>>, disjoint: Vec<&Edge<'genome>>) -> Self {
        todo!()
    }

    pub fn get(&self, edge: &Edge<'genome>) -> &Edge<'genome> {
        self.hash_set.get(edge).unwrap()
    }

    pub fn insert(&mut self, edge: Edge<'genome>) {
        let edge = Rc::new(edge);

        let inserted = self.btree_set.insert(edge.clone());
        assert!(inserted);

        let inserted = self.hash_set.insert(edge.clone());
        assert!(inserted);
    }

    // returns two random nonequal edges
    pub fn random_edges(&self, rng: &mut impl Rng) -> (&Edge<'genome>, &Edge<'genome>) {
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

    pub fn iter_ordered(&self) -> impl Iterator<Item = &Edge<'genome>> {
        self.btree_set.iter().map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn iter_unordered(&self) -> impl Iterator<Item = &Edge<'genome>> {
        self.hash_set.iter().map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn hash_difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Edge<'genome>> {
        self.hash_set.difference(&other.hash_set).map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn hash_intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Edge<'genome>> {
        self.hash_set
            .intersection(&other.hash_set)
            .map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn hash_symmetric_difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Edge<'genome>> {
        self.hash_set
            .symmetric_difference(&other.hash_set)
            .map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.btree_set.len(), self.hash_set.len());
        self.hash_set.len() // need to check if one is faster than the other
    }
}

impl fmt::Debug for Edges<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter_ordered()).finish()
    }
}
