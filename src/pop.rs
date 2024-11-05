extern crate alloc;
use crate::node::{Leading, Trailing};
use core::cell::*;
use hashbrown::HashMap;

#[thread_local]
static CONNS: LazyCell<RefCell<HashMap<(usize, usize), usize>>> = LazyCell::new(|| RefCell::new(HashMap::new()));
#[thread_local]
static NODES: Cell<usize> = Cell::new(0);

pub struct Pop;

impl Pop {
    pub fn next_conn_innov(leading: &Leading, trailing: &Trailing) -> usize {
        let key = (leading.innov(), trailing.innov());
        let mut conns = CONNS.borrow_mut();
        let next = conns.len();
        *conns.entry(key).or_insert(next)
    }

    pub fn next_node_innov() -> usize {
        let next = NODES.get();
        NODES.update(|nodes| nodes + 1);
        next
    }
}
