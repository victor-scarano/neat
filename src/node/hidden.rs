use crate::{Activation, Connection, node::{ConnectionInput, ConnectionOutput, Node}};
use std::{cell::RefCell, collections::BTreeSet, hash, rc::Rc};
use rand::Rng;

#[derive(Debug)]
pub(crate) struct Hidden {
    forward_conns: RefCell<BTreeSet<Rc<Connection>>>,
    backward_conns: RefCell<BTreeSet<Rc<Connection>>>,
    activation: Activation,
    bias: f32,
    innovation: u32,
}

impl Node for Hidden {
    fn new<R: Rng>(rng: &mut R, innovation: &crate::Innovation, config: &crate::Config) -> Self where Self: Sized {
        Self {
            forward_conns: RefCell::new(BTreeSet::new()),
            backward_conns: RefCell::new(BTreeSet::new()),
            activation: config.default_activation(),
            bias: config.new_node_bias(rng),
            innovation: innovation.new_node_innovation(),
        }
    }

    fn bias(&self) -> f32 {
        self.bias
    }

    fn activation(&self) -> Activation {
        self.activation.clone()
    }

    fn innovation(&self) -> u32 {
        self.innovation
    }
}

impl ConnectionInput for Hidden {
    fn insert_forward_conn(&self, conn: Rc<Connection>) {
        self.forward_conns.borrow_mut().insert(conn);
    }
    
    fn num_forward_conns(&self) -> usize {
        self.forward_conns.borrow().len()
    }
}

impl ConnectionOutput for Hidden {
    fn insert_backward_conn(&self, conn: Rc<Connection>) {
        self.backward_conns.borrow_mut().insert(conn);
    }

    fn num_backward_conns(&self) -> usize {
        self.backward_conns.borrow().len()
    }

    fn any_backward_conns<F: FnMut(&Rc<Connection>) -> bool>(&self, f: F) -> bool {
        self.backward_conns.borrow().iter().any(f)
    }
}

impl Eq for Hidden {}

impl hash::Hash for Hidden {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.forward_conns.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
        self.backward_conns.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
        self.activation().hash(state);
    }
}

impl PartialEq for Hidden {
    fn eq(&self, other: &Self) -> bool {
        self.forward_conns == other.forward_conns &&
        self.backward_conns == other.backward_conns &&
        self.activation() == other.activation()
    }
}

