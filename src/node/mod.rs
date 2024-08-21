use crate::{Activation, Config, Connection, Innovation};
use std::{any::Any, rc::Rc};
use rand::Rng;

mod input;
mod hidden;
mod output;

pub(crate) use input::Input;
pub(crate) use hidden::Hidden;
pub(crate) use output::Output;

pub(crate) trait Node {
    fn new<R: Rng>(rng: &mut R, innovation: &Innovation, config: &Config) -> Self where Self: Sized;
    fn bias(&self) -> f32;
    fn activation(&self) -> Activation;
    fn innovation(&self) -> u32;
}

pub(crate) trait ConnectionInput: Any + Node {
    fn insert_forward_conn(&self, conn: Rc<Connection>);
    fn num_forward_conns(&self) -> usize;
}

pub(crate) trait ConnectionOutput: Any + Node {
    fn insert_backward_conn(&self, conn: Rc<Connection>);
    fn num_backward_conns(&self) -> usize;
    fn any_backward_conns<F: FnMut(&Rc<Connection>) -> bool>(&self, f: F) -> bool;
}

