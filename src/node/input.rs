extern crate alloc;

use crate::{node::*, pop::Pop};
use core::fmt;
use alloc::rc::Rc;

pub struct Input {
    innov: usize,
    pub idx: usize,
    bias: f32,
}

impl Input {
    pub fn new(idx: usize) -> Rc<Self> {
        Rc::new(Self {
            innov: Pop::next_node_innov(),
            idx,
            bias: f32::NAN,
        })
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

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Input Node")
            .field("Bias", &self.bias)
            .field("Innovation", &self.innov)
            .finish()
    }
}
