use crate::{node::*, population::Population};
use std::fmt;

pub struct Input {
    bias: f32,
    innov: usize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            bias: f32::NAN,
            innov: Population::next_node_innov(),
        }
    }
}

impl Node for Input {
    fn level(&self) -> usize {
        0
    }

    fn bias(&self) -> f32 {
        self.bias
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl Leadingable for Input {}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Input Node")
            .field("Bias", &self.bias)
            .field("Innovation", &self.innov)
            .finish()
    }
}
