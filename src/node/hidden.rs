use crate::{Activation, Conn, node::{ConnInput, ConnOutput, Node}};
use std::{any::Any, cell::RefCell, collections::BTreeSet, hash, rc::Rc};
use rand::Rng;

#[derive(Debug)]
pub(crate) struct Hidden {
    forward_conns: RefCell<BTreeSet<Rc<Conn>>>,
    backward_conns: RefCell<BTreeSet<Rc<Conn>>>,
    activation: Activation,
    bias: f32,
    innovation: u32,
}

impl Node for Hidden {
    fn new<R: Rng>(rng: &mut R, innovation: &crate::Innov, config: &crate::Config) -> Self where Self: Sized {
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

impl ConnInput for Hidden {
    fn iter_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Conn>>> {
        Box::new(self.forward_conns.borrow().iter().cloned().collect::<Vec<_>>().into_iter())
    }

    fn iter_enabled_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Conn>>> {
        Box::new(self.forward_conns.borrow().iter().filter(|connection| {
            connection.enabled()
        }).cloned().collect::<Vec<_>>().into_iter())
    }

    fn insert_forward_conn(&self, conn: Rc<Conn>) {
        self.forward_conns.borrow_mut().insert(conn);
    }
    
    fn num_forward_conns(&self) -> usize {
        self.forward_conns.borrow().len()
    }
}

impl ConnOutput for Hidden {
    fn iter_backward_conns(&self) -> Box<dyn Iterator<Item = Rc<Conn>>> {
        Box::new(self.backward_conns.borrow().iter().cloned().collect::<Vec<_>>().into_iter())
    }

    fn insert_backward_conn(&self, conn: Rc<Conn>) {
        self.backward_conns.borrow_mut().insert(conn);
    }

    fn num_backward_conns(&self) -> usize {
        self.backward_conns.borrow().len()
    }

    fn contains_backward_conn_by(&self, f: &mut dyn FnMut(Rc<Conn>) -> bool) -> bool where Self: Sized {
        true
    }
}

impl Eq for Hidden {}

impl hash::Hash for Hidden {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.activation().hash(state);
        self.forward_conns.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
        self.backward_conns.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
    }
}

impl PartialEq for Hidden {
    fn eq(&self, other: &Self) -> bool {
        self.activation() == other.activation() &&
        self.forward_conns == other.forward_conns &&
        self.backward_conns == other.backward_conns
    }
}

