use crate::Conn;
use std::{cell::RefCell, hash, slice};
use rand::Rng;

mod input;
mod hidden;
mod output;

pub(crate) use input::Input;
pub(crate) use hidden::Hidden;
pub(crate) use output::Output;

pub(crate) trait Node {
    fn new<R: Rng>(rng: &mut R) -> Self where Self: Sized;
    fn innov(&self) -> usize;
}

pub(crate) trait ConnInput<'g>: Node {
    // might need to change the name to insert_forward_conn
    fn insert_conn(&mut self, conn: &'g RefCell<Conn<'g>>);
    fn num_conns(&self) -> usize;
    fn iter_conns(&self) -> slice::Iter<&'g RefCell<Conn<'g>>>;
}

pub(crate) trait ConnOutput<'g>: Node {}

impl<'g> hash::Hash for dyn ConnInput<'g> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl<'g> hash::Hash for dyn ConnOutput<'g> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}
