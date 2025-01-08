extern crate alloc;
use crate::node::*;
use core::fmt;
use alloc::rc::Rc;

#[derive(Eq, Clone, Debug, Hash, PartialEq)]
pub enum Head {
    Hidden(RawHidden),
    Output(RawOutput),
}

impl Head {
    pub fn hidden(&self) -> Option<&Hidden> {
        match self {
            Self::Hidden(hidden) => Some(hidden.as_ref()),
            Self::Output(_) => None,
        }
    }

    pub fn output(&self) -> Option<&Output> {
        match self {
            Self::Hidden(_) => None,
            Self::Output(output) => Some(output.as_ref()),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.as_ref().innov(),
            Self::Output(output) => output.as_ref().innov(),
        }
    }
}

impl Node for Head {
    fn layer(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.as_ref().layer(),
            Self::Output(output) => output.as_ref().layer(),
        }
    }

    fn bias(&self) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.as_ref().bias(),
            Self::Output(output) => output.as_ref().bias(),
        }
    }

    fn innov(&self) -> usize {
        match self {
            Self::Hidden(hidden) => hidden.as_ref().innov(),
            Self::Output(output) => output.as_ref().innov(),
        }
    }

    fn update_layer(&self, layer: usize) {
        match self {
            Self::Hidden(hidden) => hidden.as_ref().update_layer(layer),
            Self::Output(output) => output.as_ref().update_layer(layer),
        }
    }

    fn activate(&self, x: f32) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.as_ref().activate(x),
            Self::Output(output) => output.as_ref().activate(x),
        }
    }

    fn response(&self) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.as_ref().response(),
            Self::Output(output) => output.as_ref().response(),
        }
    }

    fn aggregator(&self) -> fn(&[f32]) -> f32 {
        match self {
            Self::Hidden(hidden) => hidden.as_ref().aggregator(),
            Self::Output(output) => output.as_ref().aggregator(),
        }
    }
}

impl fmt::Pointer for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // does slapping a reference before the node give the address of the actual node? it compiles for now
            Self::Hidden(ref hidden) => fmt::Pointer::fmt(&hidden, f),
            Self::Output(ref output) => fmt::Pointer::fmt(&output, f),
        }
    }
}

impl From<&Hidden> for Head {
    fn from(value: &Hidden) -> Self {
        Self::Hidden(value.into())
    }
}

impl From<&Output> for Head {
    fn from(value: &Output) -> Self {
        Self::Output(value.into())
    }
}

impl PartialEq<Hidden> for Head {
    fn eq(&self, rhs: &Hidden) -> bool {
        self.hidden().map(|lhs| lhs == rhs).is_some()
    }
}

impl PartialEq<Output> for Head {
    fn eq(&self, rhs: &Output) -> bool {
        self.output().map(|lhs| lhs == rhs).is_some()
    }
}

