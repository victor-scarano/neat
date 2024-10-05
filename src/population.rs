use crate::node::{Leading, Trailing};
use std::{cell::{Cell, RefCell}, collections::HashMap};

thread_local! {
    static CONNS: RefCell<HashMap<(usize, usize), usize>> = Default::default();
    static NODES: Cell<usize> = Default::default();
}

pub struct Population;

impl Population {
    pub fn next_conn_innov(leading: &Leading, trailing: &Trailing) -> usize {
        let key = (leading.innov(), trailing.innov());
        CONNS.with(|conns| {
            let mut conns = conns.borrow_mut();
            let next = conns.len();
            *conns.entry(key).or_insert(next)
        })
    }

    pub fn next_node_innov() -> usize {
        let next = NODES.get();
        NODES.with(|nodes| nodes.update(|nodes| nodes + 1));
        next
    }
}
