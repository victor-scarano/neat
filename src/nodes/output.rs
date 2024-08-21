use crate::{nodes::{ConnectionOutput, Node}, Activation, Config, Connection, Innovation};
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

pub(crate) struct Output {
    backward_conns: RefCell<BTreeSet<Rc<Connection>>>,
    activation: Activation,
    bias: f32,
    innovation: u32,
}

impl Node for Output {
    fn new<R: rand::Rng>(rng: &mut R, innovation: &Innovation, config: &Config) -> Self where Self: Sized {
        Self {
            backward_conns: RefCell::new(BTreeSet::new()),
            activation: config.default_activation(),
            bias: config.new_node_bias(rng),
            innovation: innovation.new_node(),
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

impl ConnectionOutput for Output {
    fn insert_backward_conn(&self, conn: Rc<Connection>) {
        self.backward_conns.borrow_mut().insert(conn);
    }

    fn num_backward_conns(&self) -> usize {
        self.backward_conns.borrow().len()
    }
}
