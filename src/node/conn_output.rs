use crate::node::*;
use std::cmp::Ordering;

#[derive(Eq, Clone, Debug, PartialEq)]
pub(crate) enum ConnOutput<'genome> {
    Hidden(&'genome Hidden),
    Output(&'genome Output),
}

impl<'genome> ConnOutput<'genome> {
    pub(crate) fn hidden(&self) -> Option<&'genome Hidden> {
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

impl ConnOutputable for ConnOutput<'_> {
    fn level(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.level(),
            Self::Output(output) => output.level(),
        }
    }

    fn activate(&self, x: f32) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.activate(x),
            Self::Output(output) => output.activate(x),
        }
    }
}

impl<'genome> From<&'genome Hidden> for ConnOutput<'genome> {
    fn from(value: &'genome Hidden) -> Self {
        Self::Hidden(value)
    }
}

impl<'genome> From<&'genome Output> for ConnOutput<'genome> {
    fn from(value: &'genome Output) -> Self {
        Self::Output(value)
    }
}

impl Ord for ConnOutput<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
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

impl PartialOrd for ConnOutput<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
