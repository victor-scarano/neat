use crate::{Conn, node::*, Population};
use std::{cell::{Ref, RefCell}, hash, slice};
use rand::Rng;

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Hidden<'g> {
    conns: RefCell<Vec<&'g Conn<'g>>>,
    innov: usize,
}

impl<'g> Hidden<'g> {
    pub(crate) fn new(rng: &mut impl Rng) -> Self {
        Self {
            conns: RefCell::new(Vec::new()),
            innov: Population::next_node_innov(),
        }
    }
}

impl<'g> Node for Hidden<'g> {
    fn new<R: Rng>(rng: &mut R) -> Self {
        Self {
            conns: RefCell::new(Vec::new()),
            innov: Population::next_node_innov(),
        }
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl<'g> InternalConnInput<'g> for Hidden<'g> {
    fn insert_conn(&self, conn: &'g Conn<'g>) {
        self.conns.borrow_mut().push(conn);
    }

    fn conns(&self) -> Ref<Vec<&'g Conn<'g>>> {
        self.conns.borrow()
    }
}

impl<'g> InternalConnOutput for Hidden<'g> {}

impl<'g> hash::Hash for Hidden<'g> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
