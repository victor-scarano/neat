use crate::{node::*, Population};
use std::{cell::Cell, hash};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Hidden {
    level: usize,
    activation: Cell<fn(f32) -> f32>,
    bias: f32,
    innov: usize,
}

impl Hidden {
    pub(crate) fn new(rng: &mut impl Rng) -> Self {
        Self {
            level: usize::MAX,
            activation: Cell::new(|_| f32::NAN),
            bias: f32::NAN,
            innov: Population::next_node_innov(),
        }
    }
}

impl Node for Hidden {
    fn bias(&self) -> f32 {
        self.bias
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl ConnInputable for Hidden {}

impl ConnOutputable for Hidden {
    fn level(&self) -> usize {
        self.level
    }

    fn activate(&self, x: f32) -> f32 {
        self.activation.get()(x)
    }
}

impl Eq for Hidden {}

impl hash::Hash for Hidden {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
