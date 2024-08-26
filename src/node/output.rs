use crate::{node::{ConnOutput, Node}, Activation, Config, Conn, Innov};
use std::{collections::BTreeSet, hash, sync::{Arc, RwLock}};
use rand::Rng;

#[derive(Debug)]
pub(crate) struct Output {
    backward_conns: RwLock<BTreeSet<Arc<Conn>>>,
    activation: Activation,
    bias: f32,
    response: f32,
    innov: u32,
}

impl Node for Output {
    fn new<R>(rng: &mut R, innov: Arc<Innov>, config: Arc<Config>) -> Self
    where
        Self: Sized,
        R: Rng
    {
        Self {
            backward_conns: RwLock::new(BTreeSet::new()),
            activation: config.default_activation(),
            bias: config.new_node_bias(rng),
            response: f32::MAX, // config.new_node_response(),
            innov: innov.new_node_innov(),
        }
    }

    fn activation(&self, x: f32) -> f32 {
        self.activation.call(x)
    }

    fn bias(&self) -> f32 {
        self.bias
    }

    fn response(&self) -> f32 {
        self.response
    }

    fn innov(&self) -> u32 {
        self.innov
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
}

impl Eq for Output {}

impl hash::Hash for Output {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.activation.hash(state);
        self.backward_conns.read().unwrap().iter().for_each(|node| Arc::as_ptr(node).hash(state));
    }
}

impl PartialEq for Output {
    fn eq(&self, other: &Self) -> bool {
        self.activation == other.activation &&
        *self.backward_conns.read().unwrap() == *other.backward_conns.read().unwrap()
    }
}

