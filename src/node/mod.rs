use crate::{Activation, Config, Connection, Innovation};
use std::{any::Any, rc::Rc};
use rand::Rng;

mod input;
mod hidden;
mod output;

pub(crate) use input::Input;
pub(crate) use hidden::Hidden;
pub(crate) use output::Output;

pub(crate) trait Node: Any {
    fn new<R: Rng>(rng: &mut R, innovation: &Innovation, config: &Config) -> Self where Self: Sized;
    fn bias(&self) -> f32;
    fn activation(&self) -> Activation;
    fn innovation(&self) -> u32;
}

pub(crate) trait ConnectionInput: Node {
    fn iter_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Connection>>>;
    fn iter_enabled_forward_conns(&self) -> Box<dyn Iterator<Item = Rc<Connection>>>;
    fn insert_forward_conn(&self, conn: Rc<Connection>);
    fn num_forward_conns(&self) -> usize;
}

pub(crate) trait ConnectionOutput: Node {
    fn insert_backward_conn(&self, conn: Rc<Connection>);
    fn num_backward_conns(&self) -> usize;
    fn contains_backward_conn_by(&self, f: &mut dyn FnMut(Rc<Connection>) -> bool) -> bool;
}

impl PartialEq for dyn ConnectionInput {
    fn eq(&self, other: &dyn ConnectionInput) -> bool {
        self.type_id() == other.type_id() &&
        self.num_forward_conns() == other.num_forward_conns() &&
        self.iter_forward_conns().zip(other.iter_forward_conns()).all(|(a, b)| Rc::ptr_eq(&a, &b))
    }
}

