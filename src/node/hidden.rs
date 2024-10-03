use crate::{Conn, node::*, Population};
use std::{cell::Cell, cmp, hash};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Hidden {
    level: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    bias: f32,
    innov: usize,
}

impl Hidden {
    pub(crate) fn new(split: &Conn) -> Self {
        let curr_level = split.input().level() + 1;
        split.output().update_level(curr_level + 1);

        Self {
            level: Cell::new(curr_level),
            activation: Cell::new(|_| f32::NAN),
            bias: f32::NAN,
            innov: Population::next_node_innov(),
        }
    }
}

impl Node for Hidden {
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

impl ConnInputable for Hidden {}

impl ConnOutputable for Hidden {
    fn update_level(&self, level: usize) {
        self.level.update(|current| cmp::max(current, level));
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
