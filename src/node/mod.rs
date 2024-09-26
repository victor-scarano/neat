use crate::Conn;
use std::{cell::{Ref, RefCell}, slice};
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

trait InternalConnInput<'g> {
    // might need to change the name to insert_forward_conn
    fn insert_conn(&self, conn: &'g Conn<'g>);
    fn conns(&self) -> Ref<Vec<&'g Conn<'g>>>;
}

trait InternalConnOutput {}
