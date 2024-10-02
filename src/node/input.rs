use crate::{Conn, node::*, Population};
use std::cell::{Ref, RefCell};
use rand::Rng;

pub(crate) struct Input<'genome> {
    forward_conns: RefCell<Vec<&'genome Conn<'genome>>>,
    bias: f32,
    innov: usize,
}

impl<'genome> Node for Input<'genome> {
    fn new<R: Rng>(rng: &mut R) -> Self {
        Self {
            forward_conns: RefCell::new(Vec::new()),
            bias: f32::NAN,
            innov: Population::next_node_innov(),
        }
    }

    fn bias(&self) -> f32 {
        self.bias
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl<'genome> ConnInputable<'genome> for Input<'genome> {
    fn insert_forward_conn(&self, conn: &'genome Conn<'genome>) {
         self.forward_conns.borrow_mut().push(conn);
    }

    fn forward_conns(&self) -> Ref<Vec<&'genome Conn<'genome>>> {
        self.forward_conns.borrow()
    }
}
