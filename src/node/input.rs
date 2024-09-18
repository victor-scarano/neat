use crate::{Conn, node::{Node, ConnInput}, Population};
use std::{cell::RefCell, slice};

pub(crate) struct Input<'g> {
    conns: Vec<&'g RefCell<Conn<'g>>>,
    innov: usize,
}

impl Node for Input<'_> {
    fn new<R: rand::Rng>(_rng: &mut R) -> Self where Self: Sized {
        Self {
            conns: Vec::new(),
            innov: Population::next_node_innov(),
        }
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl<'g> ConnInput<'g> for Input<'g> {
    fn insert_conn(&mut self, conn: &'g RefCell<Conn<'g>>) {
         self.conns.push(conn);
    }

    fn num_conns(&self) -> usize {
        self.conns.len()
    }

    fn iter_conns(&self) -> slice::Iter<&'g RefCell<Conn<'g>>> {
        self.conns.iter()
    }
}
