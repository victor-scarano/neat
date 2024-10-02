use crate::node::*;
use std::cmp::Ordering;

#[derive(Eq, Clone, Debug, PartialEq)]
pub(crate) enum ConnOutput<'genome> {
    Hidden(&'genome Hidden<'genome>),
    Output(&'genome Output),
}

impl<'genome> ConnOutput<'genome> {
    pub(crate) fn hidden(&self) -> Option<&'genome Hidden<'genome>> {
        match self {
            Self::Hidden(hidden) => Some(*hidden),
            Self::Output(_) => None,
        }
    }

    fn output(&self) -> Option<&'genome Output> {
        match self {
            Self::Hidden(_) => None,
            Self::Output(output) => Some(*output),
        }
    }

    pub(crate) fn innov(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.innov(),
            Self::Output(output) => output.innov(),
        }
    }
}

impl<'genome> ConnOutputable for ConnOutput<'genome> {
    fn inc_backward_conns(&self) {
        match self {
            Self::Hidden(hidden) => hidden.inc_backward_conns(),
            Self::Output(output) => output.inc_backward_conns(),
        }
    }

    fn num_backward_conns(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.num_backward_conns(),
            Self::Output(output) => output.num_backward_conns(),
        }
    }

    fn activate(&self, x: f32) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.activate(x),
            Self::Output(output) => output.activate(x),
        }
    }
}

impl<'genome> From<&'genome Hidden<'genome>> for ConnOutput<'genome> {
    fn from(value: &'genome Hidden<'genome>) -> Self {
        Self::Hidden(value)
    }
}

impl<'genome> From<&'genome Output> for ConnOutput<'genome> {
    fn from(value: &'genome Output) -> Self {
        Self::Output(value)
    }
}

impl<'genome> Ord for ConnOutput<'genome> {
    fn cmp(&self, other: &Self) -> Ordering {
        // self is hidden && other is output -> Less
        // self is output && other is hidden -> Greater
        // self is hidden && other is hidden -> most back conns
        todo!();
    }
}

impl<'genome> PartialEq<ConnInput<'genome>> for ConnOutput<'genome> {
    fn eq(&self, other: &ConnInput<'genome>) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), ConnInput::Hidden(rhs)) => lhs == rhs,
            _ => false
        }
    }
}

impl<'genome> PartialOrd for ConnOutput<'genome> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
