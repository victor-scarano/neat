extern crate alloc;
use crate::{conn::Conn, pop::Pop};
use core::{cell::Cell, cmp, hash, ptr};
use alloc::{rc::Rc, vec::Vec};
use hashbrown::HashMap;

pub enum Accum {
    Elems(Vec<f32>),
    Eval(f32),
}

impl Accum {
    pub fn new() -> Self {
        Self::Elems(Vec::new())
    }

    pub fn push(&mut self, value: f32) {
        match self {
            Self::Elems(elems) => elems.push(value),
            Self::Eval(_) => panic!(),
        }
    }

    pub fn eval(&mut self, f: fn(&[f32]) -> f32) -> f32 {
        match self {
            Self::Elems(elems) => {
                let eval = f(elems);
                *self = Self::Eval(eval);
                eval
            }
            Self::Eval(eval) => *eval
        }
    }
}

pub trait Node {
    /// Returns the [`Node`]'s `layer` in the [`Genome`].
    fn layer(&self) -> usize;

    /// Returns the [`Node`]'s `bias`.
    fn bias(&self) -> f32;

    /// Returns the [`Node`]'s `innov`.
    fn innov(&self) -> usize;

    /// Updates the [`Node`]'s `layer` in the [`Genome`].
    ///
    /// # Panics
    /// Panics if called on an [`Input`] node.
    fn update_layer(&self, layer: usize);

    /// Returns the [`Node`]'s activation based on the given input.
    fn activate(&self, x: f32) -> f32;

    /// Returns the [`Node`]'s `response`.
    fn response(&self) -> f32;

    /// Returns the [`Node`]'s aggregation function.
    fn aggregator(&self) -> fn(&[f32]) -> f32;
}

#[derive(Debug)]
pub struct Input {
    innov: usize,
    pub idx: usize,
    bias: f32,
}

impl Input {
    pub fn new(idx: usize) -> Rc<Self> {
        Rc::new(Self { innov: Pop::next_node_innov(), idx, bias: f32::default() })
    }

    pub fn eval<const I: usize>(&self, weight: f32, inputs: [f32; I]) -> f32 {
        weight * (self.bias() + inputs[self.idx])
    }
}

impl Node for Input {
    fn layer(&self) -> usize { 0 }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { panic!(); }
    fn activate(&self, x: f32) -> f32 { panic!(); }
    fn response(&self) -> f32 { panic!(); }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { panic!(); }
}

#[derive(Clone, Debug)]
pub struct Hidden {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Hidden {
    pub fn new(conn: &Conn) -> Rc<Self> {
        let curr_level = conn.leading.layer();
        conn.trailing.update_layer(curr_level + 1);

        Rc::new(Self {
            layer: Cell::new(curr_level),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
        })
    }

    pub fn eval(self: &Rc<Self>, weight: f32, map: &mut HashMap<Trailing, Accum>) -> f32 {
        let input = map.get_mut(&Trailing::from(self)).unwrap().eval(self.aggregator);
        weight * self.activate(self.bias() + (self.response() * input))
    }
}

impl Node for Hidden {
    fn layer(&self) -> usize { self.layer.get() }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { self.layer.update(|current| cmp::max(current, layer)); }
    fn activate(&self, x: f32) -> f32 { self.activation.get()(x) }
    fn response(&self) -> f32 { self.response }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { self.aggregator }
}

impl Eq for Hidden {}

impl hash::Hash for Hidden {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.response.to_le_bytes().hash(state);
        self.bias.to_le_bytes().hash(state);
        self.innov.hash(state);
    }
}

impl PartialEq for Hidden {
    fn eq(&self, other: &Self) -> bool {
        self.response == other.response && self.bias == other.bias && self.innov == other.innov
    }
}

#[derive(Debug, PartialEq)]
pub struct Output {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Output {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            layer: 1.into(),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov: Pop::next_node_innov(),
        })
    }

    pub fn eval(self: &Rc<Self>, map: &mut HashMap<Trailing, Accum>) -> f32 {
        let input = map.get_mut(&Trailing::from(self)).unwrap().eval(self.aggregator);
        self.activate(self.bias() + (self.response() * input))
    }
}

impl Node for Output {
    fn layer(&self) -> usize { self.layer.get() }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { self.layer.update(|current| cmp::max(current, layer)); }
    fn activate(&self, x: f32) -> f32 { self.activation.get()(x)}
    fn response(&self) -> f32 { self.response }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { self.aggregator }
}

impl Eq for Output {}

impl hash::Hash for Output {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.layer.get().hash(state);
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
    fn layer(&self) -> usize {
        match self {
            Self::Input(input) => input.layer(),
            Self::Hidden(hidden) => hidden.layer(),
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

    fn update_layer(&self, layer: usize) { todo!(); }

    fn activate(&self, x: f32) -> f32 { todo!(); }

    fn response(&self) -> f32 { todo!(); }

    fn aggregator(&self) -> fn(&[f32]) -> f32 { todo!(); }
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
    fn layer(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.layer(),
            Self::Output(output) => output.layer(),
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

    fn update_layer(&self, layer: usize) {
        match self {
            Self::Hidden(hidden) => hidden.update_layer(layer),
            Self::Output(output) => output.update_layer(layer),
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

    fn aggregator(&self) -> fn(&[f32]) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.aggregator(),
            Self::Output(output) => output.aggregator(),
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

impl PartialEq<Leading> for Trailing {
    fn eq(&self, other: &Leading) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Leading::Hidden(rhs)) => lhs == rhs,
            _ => false
        }
    }
}

