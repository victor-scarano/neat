use crate::{conn::Conn, node::*, population::Population};
use std::{cell::Cell, cmp, fmt, hash};

#[derive(Clone)]
pub struct Hidden {
    level: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Hidden {
    pub fn new(conn: &Conn) -> Self {
        let curr_level = conn.leading().level();
        conn.trailing().update_level(curr_level + 1);

        Self {
            level: Cell::new(curr_level),
            activation: Cell::new(|_| f32::NAN),
            response: f32::NAN,
            bias: f32::NAN,
            innov: Population::next_node_innov(),
        }
    }
}

impl Node for Hidden {
    fn level(&self) -> usize {
        self.level.get()
    }

    fn bias(&self) -> f32 {
        self.bias
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl Leadingable for Hidden {}

impl Trailingable for Hidden {
    fn update_level(&self, level: usize) {
        self.level.update(|current| cmp::max(current, level));
    }

    fn activate(&self, x: f32) -> f32 {
        self.activation.get()(x)
    }

    fn response(&self) -> f32 {
        self.response
    }
}

impl Eq for Hidden {}

impl fmt::Debug for Hidden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hidden Node")
            .field("Level", &self.level.get())
            .field("Response", &self.response)
            .field("Bias", &self.bias)
            .field("Innovation", &self.innov)
            .finish()
    }
}

impl hash::Hash for Hidden {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // self.level.get().hash(state);
        // self.activation.get().hash(state);
        self.response.to_bits().hash(state);
        self.bias.to_bits().hash(state);
        self.innov.hash(state);
    }
}

impl PartialEq for Hidden {
    fn eq(&self, other: &Self) -> bool {
        self.response == other.response &&
            self.bias == other.bias &&
            self.innov == other.innov
    }
}
