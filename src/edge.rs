extern crate alloc;
use crate::{node::*, pop::Pop};
use core::{cell::Cell, cmp::Ordering, fmt, hash};
use alloc::{collections::BTreeSet, rc::*};
use hashbrown::HashSet;

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

    pub fn disable(&self) -> &Self {
        self.enabled.set(false);
        self
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

// TODO: Write custom Rc implementation to optimize for only two possible references so that the RcInner allocation
// isn't as large as it normally is
pub struct Edges {
    btree: BTreeSet<Rc<Edge>>,
    hash: HashSet<Rc<Edge>>,
}

impl Edges {
    pub fn new() -> Self {
        Self {
            btree: BTreeSet::new(),
            hash: HashSet::new(),
        }
    }

    pub fn from(matching: Vec<&Edge>, disjoint: Vec<&Edge>) -> Self {
        let btree = BTreeSet::from_iter(matching.into_iter()
            .chain(disjoint.into_iter())
            .map(|edge| Rc::new(edge.clone())));

        let hash = HashSet::from_iter(btree.iter().cloned());

        assert_eq!(btree.len(), hash.len());

        Self { btree, hash }
    }

    pub fn get(&self, edge: &Edge) -> &Edge {
        self.hash.get(edge).unwrap()
    }

    pub fn insert(&mut self, edge: Edge) {
        let edge = Rc::new(edge);

        let inserted = self.btree.insert(edge.clone());
        assert!(inserted);

        let inserted = self.hash.insert(edge.clone());
        assert!(inserted);
    }

    pub fn iter_ordered(&self) -> impl Iterator<Item = &Edge> {
        self.btree.iter().map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn iter_unordered(&self) -> impl Iterator<Item = &Edge> {
        self.hash.iter().map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn hash_difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Edge> {
        self.hash.difference(&other.hash).map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn hash_intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Edge> {
        self.hash
            .intersection(&other.hash)
            .map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn hash_symmetric_difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Edge> {
        self.hash
            .symmetric_difference(&other.hash)
            .map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.btree.len(), self.hash.len());
        self.hash.len()
    }
}

