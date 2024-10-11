extern crate alloc;

use crate::node::*;
use core::cmp::Ordering;
use alloc::rc::Rc;

#[derive(Eq, Clone, Debug, PartialEq)]
pub enum Trailing {
    Hidden(Rc<Hidden>),
    Output(Rc<Output>),
}

impl Trailing {
    pub fn hidden(&self) -> Option<Rc<Hidden>> {
        match self {
            Self::Hidden(hidden) => Some(hidden.clone()),
            Self::Output(_) => None,
        }
    }

    pub fn output(&self) -> Option<Rc<Output>> {
        match self {
            Self::Hidden(_) => None,
            Self::Output(output) => Some(output.clone()),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.innov(),
            Self::Output(output) => output.innov(),
        }
    }
}

impl Node for Trailing {
    fn level(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.level(),
            Self::Output(output) => output.level(),
        }
    }

    fn bias(&self) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.bias(),
            Self::Output(output) => output.bias(),
        }
    }

    fn innov(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.innov(),
            Self::Output(output) => output.innov(),
        }
    }
}

impl Trailable for Trailing {
    fn update_level(&self, level: usize) {
        match self {
            Self::Hidden(hidden) => hidden.update_level(level),
            Self::Output(output) => output.update_level(level),
        }
    }

    fn activate(&self, x: f32) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.activate(x),
            Self::Output(output) => output.activate(x),
        }
    }

    fn response(&self) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.response(),
            Self::Output(output) => output.response(),
        }
    }
}

impl From<&Rc<Hidden>> for Trailing {
    fn from(value: &Rc<Hidden>) -> Self {
        Self::Hidden(value.clone())
    }
}

impl From<&Rc<Output>> for Trailing {
    fn from(value: &Rc<Output>) -> Self {
        Self::Output(value.clone())
    }
}

impl Ord for Trailing {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!();
    }
}

impl PartialEq<Leading> for Trailing {
    fn eq(&self, other: &Leading) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Leading::Hidden(rhs)) => lhs == rhs,
            _ => false
        }
    }
}

impl PartialOrd for Trailing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
