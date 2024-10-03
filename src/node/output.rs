use crate::{node::*, Population};
use std::cell::Cell;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub(crate) struct Output {
    level: usize,
    activation: Cell<fn(f32) -> f32>,
    bias: f32,
    innov: usize,
}

impl Output {
    fn new(rng: &mut impl Rng) -> Self {
        Self {
            level: usize::MAX,
            activation: Cell::new(|_| f32::NAN),
            bias: f32::NAN,
            innov: Population::next_node_innov(),
        }
    }
}

impl Node for Output {
    fn bias(&self) -> f32 {
        self.bias
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl ConnOutputable for Output {
    fn level(&self) -> usize {
        self.level
    }

    fn activate(&self, x: f32) -> f32 {
        self.activation.get()(x)
    }
}

impl Eq for Output {}
