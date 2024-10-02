use crate::{node::*, Population};
use std::cell::Cell;
use rand::Rng;

#[derive(PartialEq)]
pub(crate) struct Output {
    num_backward_conns: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    bias: f32,
    innov: usize,
}

impl Node for Output {
    fn new<R: Rng>(rng: &mut R) -> Self {
        Self {
            num_backward_conns: Cell::new(0),
            activation: Cell::new(|_| f32::NAN),
            bias: f32::NAN,
            innov: Population::next_node_innov(),
        }
    }

    fn bias(&self) -> f32 {
        self.bias
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
    
    fn activate(&self, x: f32) -> f32 {
        self.activation.get()(x)
    }
}

impl Eq for Output {}
