use crate::{node::*, Population};
use std::cell::Cell;
use rand::Rng;

#[derive(Eq, PartialEq)]
pub(crate) struct Output {
    num_backward_conns: Cell<usize>,
    innov: usize,
}

impl Node for Output {
    fn new<R: Rng>(rng: &mut R) -> Self {
        Self {
            num_backward_conns: Cell::new(0),
            innov: Population::next_node_innov(),
        }
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl ConnOutputable for Output {
    fn inc_backward_conns(&self) {
        self.num_backward_conns.update(|curr| curr + 1);
    }

    fn num_backward_conns(&self) -> usize {
        self.num_backward_conns.get()
    }
}
