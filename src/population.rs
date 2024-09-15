use crate::node::{ConnInput, ConnOutput};
use std::{cell::{Cell, RefCell}, collections::HashMap};

thread_local! {
    static CONNS: RefCell<HashMap<(usize, usize), usize>> = Default::default();
    static NODES: Cell<usize> = Default::default();
}

pub struct Population;

impl Population {
    pub(crate) fn next_conn_innov(input: &dyn ConnInput, output: &dyn ConnOutput) -> usize {
        let key = (input.innov(), output.innov());
        CONNS.with(|conns| {
            let mut conns = conns.borrow_mut();
            let next = conns.len();
            *conns.entry(key).or_insert(next)
        })
    }

    pub(crate) fn next_node_innov() -> usize {
        let next = NODES.get();
        NODES.with(|nodes| nodes.update(|nodes| nodes + 1));
        next
    }
}
