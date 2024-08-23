use crate::{node::{ConnOutput, Node}, Activation, Config, Conn, Innov};
use std::{cell::RefCell, collections::BTreeSet, hash, rc::Rc};

#[derive(Debug)]
pub(crate) struct Output {
    backward_conns: RefCell<BTreeSet<Rc<Conn>>>,
    activation: Activation,
    bias: f32,
    innovation: u32,
}

impl Node for Output {
    fn new<R: rand::Rng>(rng: &mut R, innovation: &Innov, config: &Config) -> Self where Self: Sized {
        Self {
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

impl ConnOutput for Output {
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

impl Eq for Output {}

impl hash::Hash for Output {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.activation().hash(state);
        self.backward_conns.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
    }
}

impl PartialEq for Output {
    fn eq(&self, other: &Self) -> bool {
        self.activation == other.activation &&
        self.backward_conns == other.backward_conns
    }
}

