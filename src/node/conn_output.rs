use crate::{Conn, node::*};
use std::slice;

#[derive(Clone)]
pub(crate) enum ConnOutput<'g> {
    Hidden(&'g Hidden<'g>),
    Output(&'g Output),
}

impl<'g> ConnOutput<'g> {
    pub(crate) fn innov(&self) -> usize {
        match self {
            Self::Hidden(ref hidden) => hidden.innov(),
            Self::Output(ref output) => output.innov(),
        }
    }
}

impl<'g> InternalConnOutput for ConnOutput<'g> {}

impl<'g> From<&'g Hidden<'g>> for ConnOutput<'g> {
    fn from(value: &'g Hidden<'g>) -> Self {
        Self::Hidden(value)
    }
}

impl<'g> From<&'g Output> for ConnOutput<'g> {
    fn from(value: &'g Output) -> Self {
        Self::Output(value)
    }
}
