extern crate alloc;
use crate::{genome::Genome, node::*, pop::Pop};
use core::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    convert::Into,
    fmt,
    hash,
    iter,
    mem::{self, MaybeUninit},
    ptr::NonNull,
    slice
};
use alloc::{collections::btree_set::{self, BTreeSet}, rc::*};
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

impl hash::Hash for Edge {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.innov.hash(state);
    }
}

// what exactly should be the ord implementation?
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
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
pub struct RawEdge(NonNull<Edge>);

impl RawEdge {
    pub fn upgrade<'a>(&self) -> &'a Edge {
        unsafe { self.0.as_ref() }
    }
}

impl From<&Edge> for RawEdge {
    fn from(value: &Edge) -> Self {
        Self(NonNull::from_ref(value))
    }
}

impl From<&mut Edge> for RawEdge {
    fn from(value: &mut Edge) -> Self {
        Self(NonNull::from_mut(value))
    }
}

impl hash::Hash for RawEdge {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.upgrade().hash(state);
    }
}

impl Ord for RawEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.upgrade().cmp(other.upgrade())
    }
}

impl PartialEq for RawEdge {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade().eq(other.upgrade())
    }
}

impl PartialOrd for RawEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Edges<const CHUNK_LEN: usize = 32> {
    bump: RefCell<Bump>,
    btree: BTreeSet<RawEdge>,
    hash: HashSet<RawEdge>,
}

impl<const CHUNK_LEN: usize> Edges<CHUNK_LEN> {
    pub fn new() -> Self {
        let bump = Bump::new();
        assert_ne!(CHUNK_LEN, 0);
        bump.set_allocation_limit(Some(CHUNK_LEN * size_of::<Edge>()));

        Self {
            bump: RefCell::new(bump),
            btree: BTreeSet::new(),
            hash: HashSet::new(),
        }
    }

    pub fn get(&self, edge: &Edge) -> Option<&Edge> {
        let edge = RawEdge::from(edge);
        self.hash.get(&edge).map(RawEdge::upgrade)
    }

    pub fn insert(&mut self, edge: Edge) {
        let edge = RawEdge::from(self.bump.borrow().alloc(edge));
        assert!(self.btree.insert(edge), "edge has already been inserted");
        assert!(self.hash.insert(edge), "edge has already been inserted");
    }

    pub fn iter(&self) -> iter::Map<btree_set::Iter<'_, RawEdge>, for<'a> fn(&'a RawEdge) -> &'a Edge> {
        self.btree.iter().map(RawEdge::upgrade)
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(
            self.btree.len(),
            self.hash.len(),
            "the hashset and btreeset used in managing a genome's edges should always be the same"
        );
        self.hash.len()
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

            let chunk = unsafe { slice::from_raw_parts(ptr, self.len()) };

            let edges = bump.alloc_slice_clone(chunk).iter().map(RawEdge::from);
            btree.extend(edges.clone());
            hash.extend(edges);
        }

        for chunk in chunks {
            let chunk: &[Edge] = unsafe { mem::transmute(chunk) };

            let edges = bump.alloc_slice_clone(chunk).iter().map(RawEdge::from);
            btree.extend(edges.clone());
            hash.extend(edges);
        }

        Self { bump: RefCell::new(bump), btree, hash }
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
        f.debug_list().entries(self.iter()).finish()
    }
}

