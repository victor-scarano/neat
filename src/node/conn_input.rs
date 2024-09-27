use crate::node::*;
use std::cell::Ref;

#[derive(Clone)]
pub(crate) enum ConnInput<'genome> {
    Input(&'genome Input<'genome>),
    Hidden(&'genome Hidden<'genome>),
}

impl<'genome> ConnInput<'genome> {
    pub(crate) fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.innov(),
            Self::Hidden(hidden) => hidden.innov(),
        }
    }
}

impl<'genome> ConnInputable<'genome> for ConnInput<'genome> {
    fn insert_forward_conn(&self, conn: &'genome Conn<'genome>) {
        match self {
            Self::Input(input) => input.insert_forward_conn(conn),
            Self::Hidden(hidden) => hidden.insert_forward_conn(conn),
        }
    }

    fn forward_conns(&self) -> Ref<Vec<&'genome Conn<'genome>>> {
        match self {
            Self::Input(input) => input.forward_conns(),
            Self::Hidden(hidden) => hidden.forward_conns(),
        }
    }
}

impl<'genome> From<&'genome Input<'genome>> for ConnInput<'genome> {
    fn from(value: &'genome Input<'genome>) -> Self {
        Self::Input(value)
    }
}

impl<'genome> From<&'genome Hidden<'genome>> for ConnInput<'genome> {
    fn from(value: &'genome Hidden<'genome>) -> Self {
        Self::Hidden(value)
    }
}

impl<'genome> PartialEq<ConnOutput<'genome>> for ConnInput<'genome> {
    fn eq(&self, other: &ConnOutput<'genome>) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), ConnOutput::Hidden(rhs)) => lhs == rhs,
            _ => false
        }
    }
}
