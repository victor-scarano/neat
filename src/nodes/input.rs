use crate::{activation, nodes::{ConnectionInput, Node}, Activation, Connection};
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
use rand::Rng;

/// have no aggregation function
/// have a fixed response multiplier of 1
pub(crate) struct Input {
    forward_conns: RefCell<BTreeSet<Rc<Connection>>>,
    innovation: u32,
}

impl Node for Input {
    fn new<R: Rng>(rng: &mut R, innovation: &crate::Innovation, config: &crate::Config) -> Self where Self: Sized {
        Self {
            forward_conns: RefCell::new(BTreeSet::new()),
            innovation: innovation.new_node(),
        }
    }

    fn bias(&self) -> f32 {
        0.0
    }

    fn activation(&self) -> Activation {
        activation::Identity.into()
    }

    fn innovation(&self) -> u32 {
        self.innovation
    }
}

impl ConnectionInput for Input {
    fn insert_forward_conn(&self, conn: Rc<Connection>) {
        self.forward_conns.borrow_mut().insert(conn);
    }

    fn num_forward_conns(&self) -> usize {
        self.forward_conns.borrow().len()
    }
}

