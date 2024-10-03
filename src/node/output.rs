use crate::{node::*, population::Population};
use std::{cell::Cell, cmp};

#[derive(Debug, PartialEq)]
pub struct Output {
    level: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Output {
    pub fn new() -> Self {
        Self {
            level: 1.into(),
            activation: Cell::new(|_| f32::NAN),
            response: f32::NAN,
            bias: f32::NAN,
            innov: Population::next_node_innov(),
        }
    }
}

impl Node for Output {
    fn level(&self) -> usize {
        self.level.get()
    }

    fn bias(&self) -> f32 {
        self.bias
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl ConnOutputable for Output {
    fn update_level(&self, level: usize) {
        self.level.update(|current| cmp::max(current, level));
    }

    fn activate(&self, x: f32) -> f32 {
        self.activation.get()(x)
    }

    fn response(&self) -> f32 {
        self.response
    }
}

impl Eq for Output {}
