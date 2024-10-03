use crate::{Conn, node::*, Population};
use std::cell::{Ref, RefCell};

#[derive(Debug)]
pub(crate) struct Input<'genome> {
    conns: RefCell<Vec<&'genome Conn<'genome>>>,
    bias: f32,
    innov: usize,
}

impl<'genome> Input<'genome> {
    fn new() -> Self {
        Self {
            conns: Default::default(),
            bias: f32::NAN,
            innov: Population::next_node_innov(),
        }
    }

    pub(crate) fn conns(&self) -> Ref<Vec<&'genome Conn<'genome>>> {
        self.conns.borrow()
    }

    fn insert_conn(&self, conn: &'genome Conn<'genome>) {
        self.conns.borrow_mut().push(conn);
    }
}

impl Node for Input<'_> {
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

impl ConnInputable for Input<'_> {}
