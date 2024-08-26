use crate::{activation, node::{ConnInput, Node}, Activation, Conn, Innov, Config};
use std::{cmp::Ordering, collections::BTreeSet, hash, sync::{Arc, RwLock}};
use rand::Rng;

/// have no aggregation function
#[derive(Debug)]
pub(crate) struct Input {
    forward_conns: RwLock<BTreeSet<Arc<Conn>>>,
    innov: u32,
}

impl Node for Input {
    fn new<R>(rng: &mut R, innov: Arc<Innov>, config: Arc<Config>) -> Self
    where
        Self: Sized,
        R: Rng
    {
        Self {
            forward_conns: RwLock::new(BTreeSet::new()),
            innov: innov.new_node_innov(),
        }
    }

    fn activation(&self, x: f32) -> f32 {
        x
    }

    fn bias(&self) -> f32 {
        0.0
    }

    fn response(&self) -> f32 {
        1.0
    }

    fn innov(&self) -> u32 {
        self.innov
    }
}

impl ConnInput for Input {
    fn iter_forward_conns(&self) -> Box<dyn Iterator<Item = Arc<Conn>>> {
        Box::new(self.forward_conns.read().unwrap().iter().cloned().collect::<Vec<_>>().into_iter())
    }

    fn insert_forward_conn(&self, conn: Arc<Conn>) {
        self.forward_conns.write().unwrap().insert(conn);
    }

    fn num_forward_conns(&self) -> usize {
        self.forward_conns.read().unwrap().len()
    }
}

impl Eq for Input {}

impl hash::Hash for Input {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.forward_conns.read().unwrap().iter().for_each(|node| Arc::as_ptr(node).hash(state));
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        *self.forward_conns.read().unwrap() == *other.forward_conns.read().unwrap()
    }
}

