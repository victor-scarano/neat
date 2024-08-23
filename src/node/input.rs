use crate::{activation, node::{ConnInput, Node}, Activation, Conn};
use std::{cell::RefCell, cmp::Ordering, collections::BTreeSet, hash, rc::Rc};
use rand::Rng;

/// have no aggregation function
/// have a fixed response multiplier of 1
#[derive(Debug)]
pub(crate) struct Input {
    forward_conns: RefCell<BTreeSet<Rc<Conn>>>,
    innov: u32,
}

impl Node for Input {
    fn new<R: Rng>(rng: &mut R, innov: &crate::Innov, config: &crate::Config) -> Self where Self: Sized {
        Self {
            forward_conns: RefCell::new(BTreeSet::new()),
            innov: innov.new_node_innovation(),
        }
    }

    fn bias(&self) -> f32 {
        0.0
    }

    fn activation(&self) -> Activation {
        activation::Identity.into()
    }

    fn innovation(&self) -> u32 {
        self.innov
    }
}

impl ConnInput for Input {
    fn iter_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Conn>>> {
        Box::new(self.forward_conns.borrow().iter().cloned().collect::<Vec<_>>().into_iter())
    }

    fn iter_enabled_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Conn>>> {
        Box::new(self.forward_conns.borrow().iter().filter(|conn| {
            conn.enabled()
        }).cloned().collect::<Vec<_>>().into_iter())
    }

    fn insert_forward_conn(&self, conn: Rc<Conn>) {
        self.forward_conns.borrow_mut().insert(conn);
    }

    fn num_forward_conns(&self) -> usize {
        self.forward_conns.borrow().len()
    }
}

impl Eq for Input {}

impl hash::Hash for Input {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.activation().hash(state);
        self.forward_conns.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.forward_conns == other.forward_conns
    }
}

