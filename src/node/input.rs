use crate::{activation, node::{ConnectionInput, Node}, Activation, Connection};
use std::{cell::RefCell, cmp::Ordering, collections::BTreeSet, rc::Rc};
use rand::Rng;

/// have no aggregation function
/// have a fixed response multiplier of 1
#[derive(Debug)]
pub(crate) struct Input {
    forward_conns: RefCell<BTreeSet<Rc<Connection>>>,
    innovation: u32,
}

impl Node for Input {
    fn new<R: Rng>(rng: &mut R, innovation: &crate::Innovation, config: &crate::Config) -> Self where Self: Sized {
        Self {
            forward_conns: RefCell::new(BTreeSet::new()),
            innovation: innovation.new_node_innovation(),
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
    fn iter_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Connection>>> {
        Box::new(self.forward_conns.borrow().iter().cloned().collect::<Vec<_>>().into_iter())
    }

    fn iter_enabled_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Connection>>> {
        Box::new(self.forward_conns.borrow().iter().filter(|connection| {
            connection.enabled()
        }).cloned().collect::<Vec<_>>().into_iter())
    }

    fn insert_forward_conn(&self, conn: Rc<Connection>) {
        self.forward_conns.borrow_mut().insert(conn);
    }

    fn num_forward_conns(&self) -> usize {
        self.forward_conns.borrow().len()
    }
}

impl Eq for Input {}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Greater
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.num_forward_conns() == other.num_forward_conns() &&
        self.iter_forward_conns().zip(other.iter_forward_conns()).all(|(a, b)| Rc::ptr_eq(&a, &b))
    }
}

