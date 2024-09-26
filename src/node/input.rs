use crate::{Conn, node::*, Population};
use std::{cell::{Ref, RefCell}, slice};
use rand::Rng;

pub(crate) struct Input<'g> {
    conns: RefCell<Vec<&'g Conn<'g>>>,
    innov: usize,
}

impl<'g> Node for Input<'g> {
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

impl<'g> InternalConnInput<'g> for Input<'g> {
    fn insert_conn(&self, conn: &'g Conn<'g>) {
         self.conns.borrow_mut().push(conn);
    }

    fn conns(&self) -> Ref<Vec<&'g Conn<'g>>> {
        self.conns.borrow()
    }
}
