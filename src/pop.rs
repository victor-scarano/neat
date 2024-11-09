use crate::node::{Leading, Trailing};
use core::cell::{Cell, RefCell};
use hashbrown::HashMap;

thread_local! {
    static CONNS: RefCell<HashMap<(usize, usize), usize>> = RefCell::new(HashMap::new());
    static NODES: Cell<usize> = Cell::new(0);
}

pub struct Pop;

impl Pop {
    pub fn next_conn_innov(leading: &Leading, trailing: &Trailing) -> usize {
        let key = (leading.innov(), trailing.innov());
        CONNS.with_borrow_mut(|conns| {
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

