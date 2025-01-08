extern crate alloc;
use crate::node::*;
use core::{fmt, ptr};
use alloc::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Tail {
    Input(RawInput),
    Hidden(RawHidden),
}

impl Tail {
    pub fn input(&self) -> Option<&Input> {
        match self {
            Self::Input(input) => Some(input.as_ref()),
            Self::Hidden(_) => None,
        }
    }

    pub fn hidden(&self) -> Option<&Hidden> {
        match self {
            Self::Input(_) => None,
            Self::Hidden(hidden) => Some(hidden.as_ref()),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.as_ref().innov(),
            Self::Hidden(hidden) => hidden.as_ref().innov(),
        }
    }
}

impl Node for Tail {
    fn layer(&self) -> usize {
        match self {
            Self::Input(input) => input.as_ref().layer(),
            Self::Hidden(hidden) => hidden.as_ref().layer(),
        }
    }

    fn bias(&self) -> f32 {
        match self {
            Self::Input(input) => input.as_ref().bias(),
            Self::Hidden(hidden) => hidden.as_ref().bias(),
        }
    }

    fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.as_ref().innov(),
            Self::Hidden(hidden) => hidden.as_ref().innov(),
        }
    }

    fn update_layer(&self, layer: usize) { todo!(); }

    fn activate(&self, x: f32) -> f32 { todo!(); }

    fn response(&self) -> f32 { todo!(); }

    fn aggregator(&self) -> fn(&[f32]) -> f32 { todo!(); }
}

impl fmt::Pointer for Tail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(input) => fmt::Pointer::fmt(&input, f),
            Self::Hidden(hidden) => fmt::Pointer::fmt(&hidden, f),
        }
    }
}

impl From<&Hidden> for Tail {
    fn from(value: &Hidden) -> Self {
        Self::Hidden(value.into())
    }
}

impl PartialEq<Input> for Tail {
    fn eq(&self, rhs: &Input) -> bool {
        self.input().map(|lhs| lhs == rhs).is_some()
    }
}

impl PartialEq<Hidden> for Tail {
    fn eq(&self, rhs: &Hidden) -> bool {
        self.hidden().map(|lhs| lhs == rhs).is_some()
    }
}

impl PartialEq<Head> for Tail {
    fn eq(&self, other: &Head) -> bool {
        // are we supposed to check for ptr equality or value equality?
        self.hidden().and_then(|lhs| other.hidden().map(|rhs| ptr::eq(&lhs, &rhs))).is_some()
    }
}

