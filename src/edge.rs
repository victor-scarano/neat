extern crate alloc;
use crate::{node::*, pop::Pop};
use core::{cell::{Cell, UnsafeCell}, cmp::Ordering, fmt, hash};
use alloc::{collections::BTreeSet, rc::*};
use hashbrown::HashSet;

#[derive(Clone)]
pub struct Edge<'genome> {
    pub tail: Tail<'genome>,
    pub head: Head<'genome>,
    pub weight: f32,
    pub enabled: Cell<bool>,
    pub layer: usize,
    pub innov: usize,
}

impl<'genome> Edge<'genome> {
    pub fn new(tail: impl Into<Tail<'genome>>, head: impl Into<Head<'genome>>) -> Self {
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
    btree_set: UnsafeCell<BTreeSet<Rc<Edge<'genome>>>>,
    hash_set: UnsafeCell<HashSet<Rc<Edge<'genome>>>>,
}

impl<'genome> Edges<'genome> {
    fn btree_set(&self) -> &BTreeSet<Rc<Edge<'genome>>> {
        unsafe { &*self.btree_set.get() }
    }

    fn btree_set_mut(&self) -> &mut BTreeSet<Rc<Edge<'genome>>> {
        unsafe { &mut *self.btree_set.get() }
    }

    fn hash_set(&self) -> &HashSet<Rc<Edge<'genome>>> {
        unsafe { &*self.hash_set.get() }
    }

    fn hash_set_mut(&self) -> &mut HashSet<Rc<Edge<'genome>>> {
        unsafe { &mut *self.hash_set.get() }
    }

    pub fn new() -> Self {
        Self {
            btree_set: UnsafeCell::new(BTreeSet::new()),
            hash_set: UnsafeCell::new(HashSet::new()),
        }
    }

    pub fn from(matching: Vec<&Edge<'_>>, disjoint: Vec<&Edge<'_>>) -> Self {
        todo!()
    }

    pub fn get(&self, edge: &Edge<'genome>) -> &Edge<'genome> {
        self.hash_set().get(edge).unwrap()
    }

    pub fn insert(&self, edge: Edge<'genome>) {
        let edge = Rc::new(edge);

        let inserted = self.btree_set_mut().insert(edge.clone());
        assert!(inserted);

        let inserted = self.hash_set_mut().insert(edge.clone());
        assert!(inserted);
    }

    pub fn iter_ordered(&self) -> impl Iterator<Item = &Edge<'genome>> {
        self.btree_set().iter().map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn iter_unordered(&self) -> impl Iterator<Item = &Edge<'genome>> {
        self.hash_set().iter().map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn hash_difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Edge<'genome>> {
        self.hash_set().difference(&other.hash_set()).map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn hash_intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Edge<'genome>> {
        self.hash_set()
            .intersection(&other.hash_set())
            .map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn hash_symmetric_difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Edge<'genome>> {
        self.hash_set()
            .symmetric_difference(&other.hash_set())
            .map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.btree_set().len(), self.hash_set().len());
        self.hash_set().len() // need to check if one is faster than the other
    }
}

impl fmt::Debug for Edges<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter_ordered()).finish()
    }
}
