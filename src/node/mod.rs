use crate::Conn;
use std::cell::Ref;
use rand::Rng;

mod conn_input;
mod conn_output;
mod input;
mod hidden;
mod output;

pub(crate) use conn_input::ConnInput;
pub(crate) use conn_output::ConnOutput;
pub(crate) use input::Input;
pub(crate) use hidden::Hidden;
pub(crate) use output::Output;

pub(crate) trait Node {
    fn new<R: Rng>(rng: &mut R) -> Self where Self: Sized;
    fn innov(&self) -> usize;
}

pub(crate) trait ConnInputable<'genome> {
    // might need to change the name to insert_forward_conn
    fn insert_forward_conn(&self, conn: &'genome Conn<'genome>);
    fn forward_conns(&self) -> Ref<Vec<&'genome Conn<'genome>>>;
}

pub(crate) trait ConnOutputable {
    fn inc_backward_conns(&self);
    fn num_backward_conns(&self) -> usize;
}
