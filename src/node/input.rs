use crate::{node::*, population::Population};

#[derive(Debug)]
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

impl ConnInputable for Input {}
