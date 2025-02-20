extern crate alloc;
use crate::node::*;
use core::fmt;

#[derive(Eq, Clone, Debug, Hash, PartialEq)]
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

impl<'a> From<&'a Input> for Tail<'a> {
    fn from(value: &'a Input) -> Self {
        Self::Input(value)
    }
}

impl<'a> From<&'a Hidden> for Tail<'a> {
    fn from(value: &'a Hidden) -> Self {
        Self::Hidden(value)
    }
}

impl PartialEq<Head<'_>> for Tail<'_> {
    fn eq(&self, other: &Head) -> bool {
        self.hidden().is_some_and(|lhs| other.hidden().is_some_and(|rhs| lhs.downgrade() == rhs.downgrade()))
    }
}
