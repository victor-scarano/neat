extern crate alloc;
use crate::node::*;
use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RawHead {
    Hidden(RawHidden),
    Output(RawOutput),
}

impl RawHead {
    pub fn upgrade(&self) -> Head {
        match self {
            Self::Hidden(hidden) => hidden.upgrade().into(),
            Self::Output(output) => output.upgrade().into(),
        }
    }
}

impl fmt::Pointer for RawHead {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hidden(hidden) => fmt::Pointer::fmt(hidden, f),
            Self::Output(output) => fmt::Pointer::fmt(output, f),
        }
    }
}

#[derive(Eq, Clone, Debug, Hash, PartialEq)]
pub enum Head<'a> {
    Hidden(&'a Hidden),
    Output(&'a Output),
}

impl Head<'_> {
    pub fn downgrade(&self) -> RawHead {
        match self {
            Self::Hidden(hidden) => RawHead::Hidden(hidden.downgrade()),
            Self::Output(output) => RawHead::Output(output.downgrade())
        }
    }

    pub fn hidden(&self) -> Option<&Hidden> {
        match self {
            Self::Hidden(hidden) => Some(hidden),
            Self::Output(_) => None,
        }
    }

    pub fn output(&self) -> Option<&Output> {
        match self {
            Self::Hidden(_) => None,
            Self::Output(output) => Some(output),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.innov(),
            Self::Output(output) => output.innov(),
        }
    }
}

impl Node for Head<'_> {
    fn layer(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.layer(),
            Self::Output(output) => output.layer(),
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

    fn update_layer(&self, layer: usize) {
        match self {
            Self::Hidden(hidden) => hidden.update_layer(layer),
            Self::Output(output) => output.update_layer(layer),
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

    fn aggregator(&self) -> fn(&[f32]) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.aggregator(),
            Self::Output(output) => output.aggregator(),
        }
    }
}

impl fmt::Pointer for Head<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // does slapping a reference before the node give the address of the actual node? it compiles for now
            Self::Hidden(ref hidden) => fmt::Pointer::fmt(&hidden, f),
            Self::Output(ref output) => fmt::Pointer::fmt(&output, f),
        }
    }
}

impl<'a> From<&'a Hidden> for Head<'a> {
    fn from(value: &'a Hidden) -> Self {
        Self::Hidden(value)
    }
}

impl<'a> From<&'a Output> for Head<'a> {
    fn from(value: &'a Output) -> Self {
        Self::Output(value)
    }
}

impl PartialEq<Hidden> for Head<'_> {
    fn eq(&self, rhs: &Hidden) -> bool {
        self.hidden().map(|lhs| lhs == rhs).is_some()
    }
}

impl PartialEq<Output> for Head<'_> {
    fn eq(&self, rhs: &Output) -> bool {
        self.output().map(|lhs| lhs == rhs).is_some()
    }
}

impl PartialEq<Tail<'_>> for Head<'_> {
    fn eq(&self, other: &Tail) -> bool {
        self.hidden().is_some_and(|lhs| other.hidden().is_some_and(|rhs| lhs.downgrade() == rhs.downgrade()))
    }
}

