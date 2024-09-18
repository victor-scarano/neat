use crate::{node::{Node, ConnOutput}, Population};
use rand::Rng;

pub(crate) struct Output {
    innov: usize,
}

impl Node for Output {
    fn new<R: Rng>(rng: &mut R) -> Self where Self: Sized {
        Self {
            innov: Population::next_node_innov()
        }
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl<'g> ConnOutput<'g> for Output {}
