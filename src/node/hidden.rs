use crate::{Conn, node::{Node, ConnInput, ConnOutput}, Population};
use std::{cell::RefCell, hash, slice};
use rand::Rng;

#[derive(Eq, Clone, PartialEq)]
pub(crate) struct Hidden<'g> {
    conns: Vec<&'g Conn<'g>>,
    innov: usize,
}

impl<'g> Node for Hidden<'g> {
    fn new<R: Rng>(rng: &mut R) -> Self where Self: Sized {
        Self {
            conns: Vec::new(),
            innov: Population::next_node_innov()
        }
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl<'g> ConnInput<'g> for Hidden<'g> {
    fn insert_conn(&mut self, conn: &'g Conn<'g>) {
        self.conns.push(conn);
    }

    fn num_conns(&self) -> usize {
        self.conns.len()
    }

    fn iter_conns(&self) -> slice::Iter<&'g Conn<'g>> {
        self.conns.iter()
    }
}

impl<'g> ConnOutput<'g> for Hidden<'g> {}

impl<'g> hash::Hash for Hidden<'g> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
