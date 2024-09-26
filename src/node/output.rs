use crate::{node::*, Population};
use rand::Rng;

pub(crate) struct Output {
    innov: usize,
}

impl Output {
}

impl Node for Output {
    fn new<R: Rng>(rng: &mut R) -> Self {
        Self {
            innov: Population::next_node_innov(),
        }
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl<'g> InternalConnOutput for Output {}
