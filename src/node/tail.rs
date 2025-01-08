extern crate alloc;
use crate::node::*;
use core::{fmt, ptr};
use alloc::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum RawTail {
    Input(RawInput),
    Hidden(RawHidden),
}

pub enum Tail<'a> {
    Input(&'a Input),
    Hidden(&'a Hidden),
}

impl Tail<'_> {
    pub fn input(&self) -> Option<&Input> {
        match self {
            Self::Input(input) => Some(input),
            Self::Hidden(_) => None,
        }
    }

    pub fn hidden(&self) -> Option<&Hidden> {
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

impl Node for Tail<'_> {
    fn layer(&self) -> usize {
        match self {
            Self::Input(input) => input.layer(),
            Self::Hidden(hidden) => hidden.layer(),
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

    fn update_layer(&self, layer: usize) { todo!(); }

    fn activate(&self, x: f32) -> f32 { todo!(); }

    fn response(&self) -> f32 { todo!(); }

    fn aggregator(&self) -> fn(&[f32]) -> f32 { todo!(); }
}

impl fmt::Pointer for Tail<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(input) => fmt::Pointer::fmt(&input, f),
            Self::Hidden(hidden) => fmt::Pointer::fmt(&hidden, f),
        }
    }
}

impl From<&Hidden> for Tail<'_> {
    fn from(value: &Hidden) -> Self {
        Self::Hidden(value.into())
    }
}

impl From<RawHidden> for Tail {
    fn from(value: RawHidden) -> Self {
        Self::Hidden(value)
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

