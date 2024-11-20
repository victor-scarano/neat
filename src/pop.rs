use crate::node::{Tail, Head};
use core::cell::{Cell, RefCell};
use hashbrown::HashMap;

thread_local! {
    static CONNS: RefCell<HashMap<(usize, usize), usize>> = RefCell::new(HashMap::new());
    static NODES: Cell<usize> = Cell::new(0);
}

pub struct Pop;

impl Pop {
    pub fn next_edge_innov(tail: &Tail, head: &Head) -> usize {
        let key = (tail.innov(), head.innov());
        CONNS.with_borrow_mut(|edges| {
            let next = edges.len();
            *edges.entry(key).or_insert(next)
        })
    }

    pub fn next_node_innov() -> usize {
        let next = NODES.get();
        NODES.with(|nodes| nodes.update(|nodes| nodes + 1));
        next
    }
}   

