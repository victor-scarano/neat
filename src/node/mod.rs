use crate::{Activation, Config, Conn, Innov};
use std::{any::Any, cmp::Ordering, hash, rc::Rc};
use rand::Rng;

mod input;
mod hidden;
mod output;

pub(crate) use input::Input;
pub(crate) use hidden::Hidden;
pub(crate) use output::Output;

pub(crate) trait Node: Any {
    fn new<R: Rng>(rng: &mut R, innovation: &Innov, config: &Config) -> Self where Self: Sized;
    fn bias(&self) -> f32;
    fn activation(&self) -> Activation;
    fn innovation(&self) -> u32;
}

pub(crate) trait ConnInput: Node {
    fn iter_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Conn>>>;
    fn iter_enabled_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Conn>>>;
    fn insert_forward_conn(&self, conn: Rc<Conn>);
    fn num_forward_conns(&self) -> usize;
}

pub(crate) trait ConnOutput: Node {
    fn iter_backward_conns(&self) -> Box<dyn Iterator<Item = Rc<Conn>>>;
    fn insert_backward_conn(&self, conn: Rc<Conn>);
    fn num_backward_conns(&self) -> usize;
    fn contains_backward_conn_by(&self, f: &mut dyn FnMut(Rc<Conn>) -> bool) -> bool;
}

impl Eq for dyn Node {}

impl hash::Hash for dyn Node {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        if let Some(input) = (self as &dyn Any).downcast_ref::<Input>() {
            input.hash(state);
        } else if let Some(hidden) = (self as &dyn Any).downcast_ref::<Hidden>() {
            hidden.hash(state);
        } else if let Some(output) = (self as &dyn Any).downcast_ref::<Output>() {
            output.hash(state);
        }
    }
}

impl PartialEq for dyn Node {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
        // TODO
    }
}

impl PartialEq for dyn ConnInput {
    fn eq(&self, other: &dyn ConnInput) -> bool {
        self.type_id() == other.type_id() &&
        self.num_forward_conns() == other.num_forward_conns() &&
        self.iter_forward_conns().zip(other.iter_forward_conns()).all(|(a, b)| Rc::ptr_eq(&a, &b))
    }
}

impl PartialEq for dyn ConnOutput {
    fn eq(&self, other: &dyn ConnOutput) -> bool {
        self.type_id() == other.type_id() &&
        self.num_backward_conns() == other.num_backward_conns() &&
        self.iter_backward_conns().zip(other.iter_backward_conns()).all(|(a, b)| Rc::ptr_eq(&a, &b))
    }
}

impl Eq for dyn ConnInput {}

impl Ord for dyn ConnInput {
    fn cmp(&self, other: &Self) -> Ordering {
        match ((self as &dyn Any).downcast_ref::<Input>(), (other as &dyn Any).downcast_ref::<Input>()) {
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            _ => Ordering::Equal
        }
    }
}

impl PartialOrd for dyn ConnInput {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for dyn ConnOutput {}

impl Ord for dyn ConnOutput {
    fn cmp(&self, other: &Self) -> Ordering {
        match ((self as &dyn Any).downcast_ref::<Hidden>(), (other as &dyn Any).downcast_ref::<Hidden>()) {
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            _ => self.num_backward_conns().cmp(&other.num_backward_conns()).reverse()
        }
    }
}

impl PartialOrd for dyn ConnOutput {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

