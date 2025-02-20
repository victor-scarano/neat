extern crate alloc;
use crate::{node::*, pop::Pop};
use core::{cell::Cell, convert::Into, fmt, slice};
use alloc::collections::btree_set::BTreeSet;

#[derive(Clone, PartialEq)]
pub struct Edge<'a> {
    pub tail: Tail<'a>,
    pub head: Head<'a>,
    pub weight: f32,
    pub enabled: Cell<bool>,
    pub layer: usize,
    pub innov: usize,
}

impl<'a> Edge<'a> {
    pub fn new(tail: impl Into<Tail<'a>>, head: impl Into<Head<'a>>) -> Self {
        let tail: Tail = tail.into();
        let head: Head = head.into();

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

#[derive(Debug, Default)]
pub struct Edges<'a>(BTreeSet<Edge<'a>>);

impl Edges<'_> {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Edges<'a> {
    pub fn get(&self, edge: &Edge<'_>) -> Option<&Edge<'_>> {
        self.0.get(edge)
    }

    pub fn insert(&mut self, edge: Edge<'_>) {
        assert!(self.0.insert(edge));
    }

    pub fn iter(&self) -> slice::Iter<'_, Edge<'a>> {
        todo!()
    }

    pub fn innov_matching() {
        todo!()
    }

    pub fn innov_disjoint() {
        todo!()
    }
}

