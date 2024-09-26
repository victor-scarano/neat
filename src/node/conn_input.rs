use crate::{Conn, node::*};
use std::{cell::{Ref, RefCell}, slice};

#[derive(Clone)]
pub(crate) enum ConnInput<'g> {
    Input(&'g Input<'g>),
    Hidden(&'g Hidden<'g>),
}

impl<'g> ConnInput<'g> {
    pub(crate) fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.innov(),
            Self::Hidden(hidden) => hidden.innov(),
        }
    }
}

impl<'g> InternalConnInput<'g> for ConnInput<'g> {
    fn insert_conn(&self, conn: &'g Conn<'g>) {
        match self {
            Self::Input(input) => input.insert_conn(conn),
            Self::Hidden(hidden) => hidden.insert_conn(conn),
        }
    }

    fn conns(&self) -> Ref<Vec<&'g Conn<'g>>> {
        match self {
            Self::Input(input) => input.conns(),
            Self::Hidden(hidden) => hidden.conns(),
        }
    }
}

impl<'g> From<&'g Input<'g>> for ConnInput<'g> {
    fn from(value: &'g Input<'g>) -> Self {
        Self::Input(value)
    }
}

impl<'g> From<&'g Hidden<'g>> for ConnInput<'g> {
    fn from(value: &'g Hidden<'g>) -> Self {
        Self::Hidden(value)
    }
}
