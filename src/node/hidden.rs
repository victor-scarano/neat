use crate::{Activation, Conn, node::{ConnInput, ConnOutput, Node}, Innov, Config};
use std::{any::Any, cell::RefCell, cmp::Ordering, collections::BTreeSet, hash, sync::{Arc, RwLock}};
use rand::Rng;

#[derive(Debug)]
pub(crate) struct Hidden {
    forward_conns: RwLock<BTreeSet<Arc<Conn>>>,
    backward_conns: RwLock<BTreeSet<Arc<Conn>>>,
    activation: Activation,
    bias: f32,
    innov: u32,
}

impl Node for Hidden {
    fn new<R: Rng>(rng: &mut R, innov: Arc<Innov>, config: Arc<Config>) -> Self where Self: Sized {
        Self {
            forward_conns: RwLock::new(BTreeSet::new()),
            backward_conns: RwLock::new(BTreeSet::new()),
            activation: config.default_activation(),
            bias: config.new_node_bias(rng),
            innov: innov.new_node_innovation(),
        }
    }

    fn bias(&self) -> f32 {
        self.bias
    }

    fn activation(&self) -> Activation {
        self.activation.clone()
    }

    fn innovation(&self) -> u32 {
        self.innov
    }
}

impl ConnInput for Hidden {
    fn iter_forward_conns(&self) -> Box<dyn Iterator<Item = Arc<Conn>>> {
        Box::new(self.forward_conns.read().unwrap().iter().cloned().collect::<Vec<_>>().into_iter())
    }

    fn iter_enabled_forward_conns(&self) -> Box<dyn Iterator<Item = Arc<Conn>>> {
        Box::new(self.forward_conns.read().unwrap().iter().filter(|connection| {
            connection.enabled()
        }).cloned().collect::<Vec<_>>().into_iter())
    }

    fn insert_forward_conn(&self, conn: Arc<Conn>) {
        self.forward_conns.write().unwrap().insert(conn);
    }
    
    fn num_forward_conns(&self) -> usize {
        self.forward_conns.read().unwrap().len()
    }
}

impl ConnOutput for Hidden {
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

impl Eq for Hidden {}

impl hash::Hash for Hidden {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.activation().hash(state);
        self.forward_conns.read().unwrap().iter().for_each(|node| Arc::as_ptr(node).hash(state));
        self.backward_conns.read().unwrap().iter().for_each(|node| Arc::as_ptr(node).hash(state));
    }
}

impl Ord for Hidden {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num_backward_conns().cmp(&other.num_backward_conns()).reverse()
    }
}

impl PartialEq for Hidden {
    fn eq(&self, other: &Self) -> bool {
        self.activation() == other.activation() &&
        *self.forward_conns.read().unwrap() == *other.forward_conns.read().unwrap() &&
        *self.backward_conns.read().unwrap() == *other.backward_conns.read().unwrap()
    }
}

impl PartialOrd for Hidden {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

