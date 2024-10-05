use crate::node::*;
use std::ptr;

#[derive(Copy, Clone, Debug)]
pub enum Leading<'g> {
    Input(&'g Input),
    Hidden(&'g Hidden),
}

impl<'g> Leading<'g> {
    pub const fn input(&self) -> Option<&Input> {
        match self {
            Self::Input(input) => Some(input),
            Self::Hidden(_) => None,
        }
    }

    pub const fn hidden(&self) -> Option<&Hidden> {
        match self {
            Self::Input(_) => None,
            Self::Hidden(hidden) => Some(hidden),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.innov(),
            Self::Hidden(hidden) => hidden.innov(),
        }
    }
}

impl<'g> Node for Leading<'g> {
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

impl Leadingable for Leading<'_> {}

impl<'g> From<&'g Input> for Leading<'g> {
    fn from(value: &'g Input) -> Self {
        Self::Input(value)
    }
}

impl<'g> From<&'g Hidden> for Leading<'g> {
    fn from(value: &'g Hidden) -> Self {
        Self::Hidden(value)
    }
}

impl<'g> PartialEq<Trailing<'g>> for Leading<'g> {
    fn eq(&self, other: &Trailing<'g>) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Trailing::Hidden(rhs)) => ptr::eq(*lhs, *rhs),
            _ => false
        }
    }
}
