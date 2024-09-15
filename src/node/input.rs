use crate::{Conn, node::{Node, ConnInput}, Population};
use std::cell::{Ref, RefCell};

struct Input<'genome> {
    conns: RefCell<Vec<&'genome Conn<'genome>>>,
    innov: usize,
}

impl Node for Input<'_> {
    fn new<R: rand::Rng>(_rng: &mut R) -> Self where Self: Sized {
        Self {
            conns: RefCell::new(Vec::new()),
            innov: Population::next_node_innov(),
        }
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl<'genome> ConnInput<'genome> for Input<'genome> {
    fn insert_conn(&self, conn: &'genome Conn<'genome>) {
         self.conns.borrow_mut().push(conn);
    }

    fn num_conns(&self) -> usize {
        self.conns.borrow().len()
    }

    fn iter_conns(&self) -> Box<dyn Iterator<Item = &&'genome Conn<'genome>>> {
        Box::new(self.conns.borrow().iter())
    }
}
