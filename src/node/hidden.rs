use crate::{Conn, node::*, Population};
use std::{cell::{Cell, Ref, RefCell}, hash};
use rand::Rng;

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Hidden<'genome> {
    conns: RefCell<Vec<&'genome Conn<'genome>>>,
    num_backward_conns: Cell<usize>,
    innov: usize,
}

impl<'genome> Node for Hidden<'genome> {
    fn new<R: Rng>(rng: &mut R) -> Self {
        Self {
            conns: RefCell::new(Vec::new()),
            num_backward_conns: Cell::new(0),
            innov: Population::next_node_innov(),
        }
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl<'genome> ConnInputable<'genome> for Hidden<'genome> {
    fn insert_forward_conn(&self, conn: &'genome Conn<'genome>) {
        self.conns.borrow_mut().push(conn);
    }

    fn forward_conns(&self) -> Ref<Vec<&'genome Conn<'genome>>> {
        self.conns.borrow()
    }
}

impl<'genome> ConnOutputable for Hidden<'genome> {
    fn inc_backward_conns(&self) {
        self.num_backward_conns.update(|curr| curr + 1);
    }

    fn num_backward_conns(&self) -> usize {
        self.num_backward_conns.get()
    }
}

impl<'genome> hash::Hash for Hidden<'genome> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
