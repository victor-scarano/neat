extern crate alloc;
use crate::{conn::Conn, pop::Pop};
use core::{cell::Cell, cmp::{self, Ordering}, fmt, hash, ptr};
use alloc::rc::Rc;

pub trait Node {
    fn level(&self) -> usize;
    fn bias(&self) -> f32;
    fn innov(&self) -> usize;
}

pub trait Trailable {
    fn update_level(&self, level: usize);
    fn activate(&self, x: f32) -> f32;
    fn response(&self) -> f32;
}

pub struct Input {
    innov: usize,
    pub idx: usize,
    bias: f32,
}

impl Input {
    pub fn new(idx: usize) -> Rc<Self> {
        Rc::new(Self {
            innov: Pop::next_node_innov(),
            idx,
            bias: 0.0,
        })
    }
}

impl Node for Input {
    fn level(&self) -> usize {
        0
    }

    fn bias(&self) -> f32 {
        self.bias
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Input Node")
            .field("Bias", &self.bias)
            .field("Innovation", &self.innov)
            .field("Index", &self.idx)
            .finish()
    }
}

#[derive(Clone)]
pub struct Hidden {
    level: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    pub aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Hidden {
    pub fn new(conn: &Conn) -> Rc<Self> {
        let curr_level = conn.leading.level();
        conn.trailing.update_level(curr_level + 1);

        Rc::new(Self {
            level: Cell::new(curr_level),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
        })
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

impl Trailable for Hidden {
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

impl Ord for Hidden {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl PartialEq for Hidden {
    fn eq(&self, other: &Self) -> bool {
        self.response == other.response &&
            self.bias == other.bias &&
            self.innov == other.innov
    }
}

impl PartialOrd for Hidden {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq)]
pub struct Output {
    level: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    pub aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Output {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            level: 1.into(),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
        })
    }
}

impl Node for Output {
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

impl Trailable for Output {
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

impl Eq for Output {}

impl fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Output Node")
            .field("Level", &self.level.get())
            .field("Response", &self.response)
            .field("Bias", &self.bias)
            .field("Innovation", &self.innov)
            .finish()
    }
}

impl hash::Hash for Output {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.level.get().hash(state);
        self.response.to_bits().hash(state);
        self.bias.to_bits().hash(state);
        self.innov.hash(state);
    }
}

#[derive(Clone, Debug)]
pub enum Leading {
    Input(Rc<Input>),
    Hidden(Rc<Hidden>),
}

impl Leading {
    pub fn input(&self) -> Option<Rc<Input>> {
        match self {
            Self::Input(input) => Some(input.clone()),
            Self::Hidden(_) => None,
        }
    }

    pub fn hidden(&self) -> Option<Rc<Hidden>> {
        match self {
            Self::Input(_) => None,
            Self::Hidden(hidden) => Some(hidden.clone()),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.innov(),
            Self::Hidden(hidden) => hidden.innov(),
        }
    }
}

impl Node for Leading {
    fn level(&self) -> usize {
        match self {
            Self::Input(input) => input.level(),
            Self::Hidden(hidden) => hidden.level(),
        }
    }

    fn bias(&self) -> f32 {
        match self {
            Self::Input(input) => input.bias(),
            Self::Hidden(hidden) => hidden.bias(),
        }
    }

    fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.innov(),
            Self::Hidden(hidden) => hidden.innov(),
        }
    }
}

impl From<&Leading> for Leading {
    fn from(value: &Leading) -> Self {
        value.clone()
    }
}

impl From<&Rc<Input>> for Leading {
    fn from(value: &Rc<Input>) -> Self {
        Self::Input(value.clone())
    }
}

impl From<&Rc<Hidden>> for Leading {
    fn from(value: &Rc<Hidden>) -> Self {
        Self::Hidden(value.clone())
    }
}

impl PartialEq<Trailing> for Leading {
    fn eq(&self, other: &Trailing) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Trailing::Hidden(rhs)) => ptr::eq(lhs, rhs),
            _ => false
        }
    }
}

#[derive(Eq, Clone, Debug, Hash, PartialEq)]
pub enum Trailing {
    Hidden(Rc<Hidden>),
    Output(Rc<Output>),
}

impl Trailing {
    pub fn hidden(&self) -> Option<Rc<Hidden>> {
        match self {
            Self::Hidden(hidden) => Some(hidden.clone()),
            Self::Output(_) => None,
        }
    }

    pub fn output(&self) -> Option<Rc<Output>> {
        match self {
            Self::Hidden(_) => None,
            Self::Output(output) => Some(output.clone()),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.innov(),
            Self::Output(output) => output.innov(),
        }
    }
}

impl Node for Trailing {
    fn level(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.level(),
            Self::Output(output) => output.level(),
        }
    }

    fn bias(&self) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.bias(),
            Self::Output(output) => output.bias(),
        }
    }

    fn innov(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.innov(),
            Self::Output(output) => output.innov(),
        }
    }
}

impl Trailable for Trailing {
    fn update_level(&self, level: usize) {
        match self {
            Self::Hidden(hidden) => hidden.update_level(level),
            Self::Output(output) => output.update_level(level),
        }
    }

    fn activate(&self, x: f32) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.activate(x),
            Self::Output(output) => output.activate(x),
        }
    }

    fn response(&self) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.response(),
            Self::Output(output) => output.response(),
        }
    }
}

impl From<&Trailing> for Trailing {
    fn from(value: &Trailing) -> Self {
        value.clone()
    }
}

impl From<Rc<Hidden>> for Trailing {
    fn from(value: Rc<Hidden>) -> Self {
        Self::Hidden(value)
    }
}

impl From<Rc<Output>> for Trailing {
    fn from(value: Rc<Output>) -> Self {
        Self::Output(value)
    }
}

impl From<&Rc<Hidden>> for Trailing {
    fn from(value: &Rc<Hidden>) -> Self {
        Self::Hidden(value.clone())
    }
}

impl From<&Rc<Output>> for Trailing {
    fn from(value: &Rc<Output>) -> Self {
        Self::Output(value.clone())
    }
}

impl Ord for Trailing {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!();
    }
}

impl PartialEq<Leading> for Trailing {
    fn eq(&self, other: &Leading) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Leading::Hidden(rhs)) => lhs == rhs,
            _ => false
        }
    }
}

impl PartialOrd for Trailing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

