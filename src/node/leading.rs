use crate::node::*;
use std::ptr::{self, NonNull};

pub enum UnsafeLeading {
    Input(NonNull<Input>),
    Hidden(NonNull<Hidden>),
}

#[derive(Copy, Clone, Debug)]
pub enum Leading<'a> {
    Input(&'a Input),
    Hidden(&'a Hidden),
}

impl<'a> From<Leading<'a>> for UnsafeLeading {
    fn from(value: Leading<'a>) -> Self {
        match value {
            Leading::Input(input) => UnsafeLeading::Input(NonNull::from(input)),
            Leading::Hidden(hidden) => UnsafeLeading::Hidden(NonNull::from(hidden))
        }
    }
}

impl<'a> Leading<'a> {
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

impl<'a> Node for Leading<'a> {
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

impl<'a> From<&UnsafeLeading> for Leading<'a> {
    fn from(value: &UnsafeLeading) -> Self {
        match value {
            UnsafeLeading::Input(input) => Leading::Input(unsafe { input.as_ref() }),
            UnsafeLeading::Hidden(hidden) => Leading::Hidden(unsafe { hidden.as_ref() })
        }
    }
}

impl<'a> From<&'a Input> for Leading<'a> {
    fn from(value: &'a Input) -> Self {
        Self::Input(value)
    }
}

impl<'a> From<&'a Hidden> for Leading<'a> {
    fn from(value: &'a Hidden) -> Self {
        Self::Hidden(value)
    }
}

impl<'a> PartialEq<Trailing<'a>> for Leading<'a> {
    fn eq(&self, other: &Trailing<'a>) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Trailing::Hidden(rhs)) => ptr::eq(*lhs, *rhs),
            _ => false
        }
    }
}
