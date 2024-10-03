use crate::node::*;
use std::ptr;

#[derive(Clone, Debug)]
pub enum ConnInput<'genome> {
    Input(&'genome Input<'genome>),
    Hidden(&'genome Hidden),
}

impl<'genome> ConnInput<'genome> {
    pub const fn input(&self) -> Option<&'genome Input> {
        match self {
            Self::Input(input) => Some(*input),
            Self::Hidden(_) => None,
        }
    }

    pub const fn hidden(&self) -> Option<&'genome Hidden> {
        match self {
            Self::Input(_) => None,
            Self::Hidden(hidden) => Some(*hidden),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.innov(),
            Self::Hidden(hidden) => hidden.innov(),
        }
    }
}

impl Node for ConnInput<'_> {
    fn level(&self) -> usize {
        match self {
            Self::Input(input) => input.level(),
            Self::Hidden(hidden) => hidden.level(),
        }
    }

    fn bias(&self) -> f32 {
        match self {
            Self::Input(input) => input.bias(),
            Self::Hidden(hidden) => hidden.bias(),
        }
    }

    fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.innov(),
            Self::Hidden(hidden) => hidden.innov(),
        }
    }
}

impl ConnInputable for ConnInput<'_> {}

impl<'genome> From<&'genome Input<'genome>> for ConnInput<'genome> {
    fn from(value: &'genome Input<'genome>) -> Self {
        Self::Input(value)
    }
}

impl<'genome> From<&'genome Hidden> for ConnInput<'genome> {
    fn from(value: &'genome Hidden) -> Self {
        Self::Hidden(value)
    }
}

impl<'genome> PartialEq<ConnOutput<'genome>> for ConnInput<'genome> {
    fn eq(&self, other: &ConnOutput<'genome>) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), ConnOutput::Hidden(rhs)) => ptr::eq(*lhs, *rhs),
            _ => false
        }
    }
}
