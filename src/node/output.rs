use crate::{node::{ConnOutput, Node}, Activation, Config, Conn, Innov};
use std::{collections::BTreeSet, hash, sync::{Arc, RwLock}};

#[derive(Debug)]
pub(crate) struct Output {
    backward_conns: RwLock<BTreeSet<Arc<Conn>>>,
    activation: Activation,
    bias: f32,
    innovation: u32,
}

impl Node for Output {
    fn new<R: rand::Rng>(rng: &mut R, innovation: Arc<Innov>, config: Arc<Config>) -> Self where Self: Sized {
        Self {
            backward_conns: RwLock::new(BTreeSet::new()),
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
    fn iter_backward_conns(&self) -> Box<dyn Iterator<Item = Arc<Conn>>> {
        Box::new(self.backward_conns.read().unwrap().iter().cloned().collect::<Vec<_>>().into_iter())
    }

    fn insert_backward_conn(&self, conn: Arc<Conn>) {
        self.backward_conns.write().unwrap().insert(conn);
    }

    fn num_backward_conns(&self) -> usize {
        self.backward_conns.read().unwrap().len()
    }

    fn contains_backward_conn_by(&self, f: &mut dyn FnMut(Arc<Conn>) -> bool) -> bool where Self: Sized {
        todo!()
    }
}

impl Eq for Output {}

impl hash::Hash for Output {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.activation().hash(state);
        self.backward_conns.read().unwrap().iter().for_each(|node| Arc::as_ptr(node).hash(state));
    }
}

impl PartialEq for Output {
    fn eq(&self, other: &Self) -> bool {
        self.activation == other.activation &&
        *self.backward_conns.read().unwrap() == *other.backward_conns.read().unwrap()
    }
}

