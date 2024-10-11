extern crate alloc;

use crate::node::*;
use core::ptr;
use alloc::rc::Rc;

#[derive(Clone, Debug)]
pub enum Leading {
    Input(Rc<Input>),
    Hidden(Rc<Hidden>),
}

impl Leading {
    pub fn input(&self) -> Option<Rc<Input>> {
        match self {
            Self::Input(input) => Some(input.clone()),
            Self::Hidden(_) => None,
        }
    }

    pub fn hidden(&self) -> Option<Rc<Hidden>> {
        match self {
            Self::Input(_) => None,
            Self::Hidden(hidden) => Some(hidden.clone()),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.innov(),
            Self::Hidden(hidden) => hidden.innov(),
        }
    }
}

impl Node for Leading {
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

impl From<&Rc<Input>> for Leading {
    fn from(value: &Rc<Input>) -> Self {
        Self::Input(value.clone())
    }
}

impl From<&Rc<Hidden>> for Leading {
    fn from(value: &Rc<Hidden>) -> Self {
        Self::Hidden(value.clone())
    }
}

impl PartialEq<Trailing> for Leading {
    fn eq(&self, other: &Trailing) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Trailing::Hidden(rhs)) => ptr::eq(lhs, rhs),
            _ => false
        }
    }
}
