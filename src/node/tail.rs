use crate::node::{self, Node, Head};
use core::{fmt, ptr, pin::Pin};

type Input<'a> = Pin<&'a node::Input>;
type Hidden<'a> = Pin<&'a node::Hidden>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tail<'genome> {
    Input(Input<'genome>),
    Hidden(Hidden<'genome>),
}

impl Tail<'_> {
    pub fn input(&self) -> Option<Input> {
        match self {
            Self::Input(input) => Some(*input),
            Self::Hidden(_) => None,
        }
    }

    pub fn hidden(&self) -> Option<Hidden> {
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
            Self::Input(ref input) => fmt::Pointer::fmt(&input, f),
            Self::Hidden(ref hidden) => fmt::Pointer::fmt(&hidden, f),
        }
    }
}

impl<'genome> From<Hidden<'genome>> for Tail<'genome> {
    fn from(value: Hidden<'genome>) -> Self {
        Self::Hidden(value)
    }
}

impl PartialEq<Input<'_>> for Tail<'_> {
    fn eq(&self, rhs: &Input) -> bool {
        self.input().and_then(|lhs| Some(lhs == *rhs)).is_some()
    }
}

impl PartialEq<Hidden<'_>> for Tail<'_> {
    fn eq(&self, rhs: &Hidden) -> bool {
        self.hidden().and_then(|lhs| Some(lhs == *rhs)).is_some()
    }
}

impl PartialEq<Head<'_>> for Tail<'_> {
    fn eq(&self, other: &Head) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Head::Hidden(rhs)) => ptr::eq(lhs, rhs),
            _ => false
        }
    }
}

