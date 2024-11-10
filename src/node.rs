extern crate alloc;
use crate::{conn::Conn, pop::Pop};
use core::{cell::Cell, cmp, fmt, hash, ptr};
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
    fn layer(&self) -> usize;
    fn bias(&self) -> f32;
    fn innov(&self) -> usize;
    fn update_layer(&self, layer: usize);
    fn activate(&self, x: f32) -> f32;
    fn response(&self) -> f32;
    fn aggregator(&self) -> fn(&[f32]) -> f32;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    innov: usize,
    bias: f32,
}

impl Input {
    pub fn new(innov: usize) -> Rc<Self> {
        Pop::next_node_innov();
        Rc::new(Self { innov, bias: f32::default() })
    }

    // we can use self.innov as the idx for any input node
    pub fn idx(&self) -> usize {
        self.innov
    }

    pub fn eval<const I: usize>(&self, weight: f32, inputs: [f32; I]) -> f32 {
        weight * (self.bias() + inputs[self.idx()])
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

#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Output {
    pub fn new(innov: usize) -> Rc<Self> {
        Pop::next_node_innov();
        Rc::new(Self {
            layer: 1.into(),
            activation: Cell::new(|x| x),
            aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
            response: 1.0,
            bias: 0.0,
            innov,
        })
    }

    pub fn idx<const I: usize>(&self) -> usize {
        self.innov - I
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

impl fmt::Pointer for Leading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(ref input) => fmt::Pointer::fmt(input, f),
            Self::Hidden(ref hidden) => fmt::Pointer::fmt(hidden, f),
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

impl PartialEq<Input> for Leading {
    fn eq(&self, rhs: &Input) -> bool {
        self.input().and_then(|lhs| Some(*lhs == *rhs)).is_some()
    }
}

impl PartialEq<Hidden> for Leading {
    fn eq(&self, rhs: &Hidden) -> bool {
        self.hidden().and_then(|lhs| Some(*lhs == *rhs)).is_some()
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

impl fmt::Pointer for Trailing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hidden(ref hidden) => fmt::Pointer::fmt(hidden, f),
            Self::Output(ref output) => fmt::Pointer::fmt(output, f),
        }
    }
}

impl From<&Trailing> for Trailing {
    fn from(value: &Trailing) -> Self {
        value.clone()
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

impl PartialEq<Hidden> for Trailing {
    fn eq(&self, rhs: &Hidden) -> bool {
        self.hidden().and_then(|lhs| Some(*lhs == *rhs)).is_some()
    }
}

impl PartialEq<Output> for Trailing {
    fn eq(&self, rhs: &Output) -> bool {
        self.output().and_then(|lhs| Some(*lhs == *rhs)).is_some()
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

// kinda ugly and not very ideal, but for now its needed for graphviz (without using dynamic dispatch)
// maybe there is a better way we can manage nodes in general to avoid all of these types.
// if preferably call this type node, but then the node trait would be conflicting
#[derive(Clone)]
pub enum AnyNode {
    Input(Rc<Input>),
    Hidden(Rc<Hidden>),
    Output(Rc<Output>),
}

impl From<Leading> for AnyNode {
    fn from(value: Leading) -> Self {
        match value {
            Leading::Input(input) => Self::Input(input),
            Leading::Hidden(hidden) => Self::Hidden(hidden),
        }
    }
}

impl From<Trailing> for AnyNode {
    fn from(value: Trailing) -> Self {
        match value {
            Trailing::Hidden(hidden) => Self::Hidden(hidden),
            Trailing::Output(output) => Self::Output(output),
        }
    }
}

impl From<&Rc<Input>> for AnyNode {
    fn from(value: &Rc<Input>) -> Self {
        Self::Input(value.clone())
    }
}

impl From<&Rc<Hidden>> for AnyNode {
    fn from(value: &Rc<Hidden>) -> Self {
        Self::Hidden(value.clone())
    }
}

impl From<&Rc<Output>> for AnyNode {
    fn from(value: &Rc<Output>) -> Self {
        Self::Output(value.clone())
    }
}

impl Node for AnyNode {
    fn bias(&self) -> f32 {
        todo!()
    }

    fn layer(&self) -> usize {
        todo!()
    }

    fn innov(&self) -> usize {
        match self {
            Self::Input(ref input) => input.innov,
            Self::Hidden(ref hidden) => hidden.innov,
            Self::Output(ref output) => output.innov,
        }
    }
    
    fn activate(&self, x: f32) -> f32 {
        todo!()
    }

    fn response(&self) -> f32 {
        todo!()
    }

    fn aggregator(&self) -> fn(&[f32]) -> f32 {
        todo!()
    }

    fn update_layer(&self, layer: usize) {
        todo!()
    }
}

impl fmt::Pointer for AnyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(ref input) => fmt::Pointer::fmt(input, f),
            Self::Hidden(ref hidden) => fmt::Pointer::fmt(hidden, f),
            Self::Output(ref output) => fmt::Pointer::fmt(output, f),
        }
    }
}

