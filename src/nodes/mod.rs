use crate::{Activation, Config, Connection, Innovation};
use std::rc::Rc;
use rand::Rng;

mod input;
mod hidden;
mod output;

pub(crate) trait Node {
    fn new<R: Rng>(rng: &mut R, innovation: &Innovation, config: &Config) -> Self where Self: Sized;
    fn bias(&self) -> f32;
    fn activation(&self) -> Activation;
    fn innovation(&self) -> u32;
}

pub(crate) trait ConnectionInput {
    fn insert_forward_conn(&self, conn: Rc<Connection>);

    fn num_forward_conns(&self) -> usize;
}

pub(crate) trait ConnectionOutput {
    fn insert_backward_conn(&self, conn: Rc<Connection>);

    fn num_backward_conns(&self) -> usize;
}
