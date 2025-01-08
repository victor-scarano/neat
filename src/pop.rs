//! NOTE: This implementation does not work on Windows and may or may not work
//! on MacOS.

extern crate alloc;
use crate::node::{Tail, Head};
use core::cell::*;
use hashbrown::HashMap;

#[thread_local]
static EDGES: LazyCell<RefCell<HashMap<(usize, usize), usize>>> = LazyCell::new(Default::default);
#[thread_local]
static NODES: Cell<usize> = Cell::new(0);

pub struct Pop;

impl Pop {
    pub fn next_edge_innov(tail: &Tail, head: &Head) -> usize {
        let key = (tail.innov(), head.innov());
        let mut edges = EDGES.borrow_mut();
        let next = edges.len();
        *edges.entry(key).or_insert(next)
    }

    pub fn next_node_innov() -> usize {
        let next = NODES.get();
        NODES.update(|nodes| nodes + 1);
        next
    }
}

