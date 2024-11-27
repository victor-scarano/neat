extern crate alloc;
use crate::node::*;
use core::fmt;
use alloc::rc::Rc;
use bumpalo::Bump;

#[derive(Eq, Clone, Debug, Hash, PartialEq)]
pub enum Head<'genome> {
    Hidden(Rc<Hidden, &'genome Bump>),
    Output(Rc<Output, &'genome Bump>),
}

impl Head {
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

impl Node for Head {
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

impl fmt::Pointer for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // does slapping a reference before the node give the address of the actual node? it compiles for now
            Self::Hidden(ref hidden) => fmt::Pointer::fmt(&hidden, f),
            Self::Output(ref output) => fmt::Pointer::fmt(&output, f),
        }
    }
}

impl From<Rc<Hidden>> for Head {
    fn from(value: Rc<Hidden>) -> Self {
        Self::Hidden(value)
    }
}

impl From<Rc<Output, &Bump>> for Head {
    fn from(value: Rc<Output, &Bump>) -> Self {
        Self::Output(value)
    }
}

impl PartialEq<Rc<Hidden>> for Head {
    fn eq(&self, rhs: &Rc<Hidden>) -> bool {
        self.hidden().and_then(|lhs| Some(lhs == *rhs)).is_some()
    }
}

impl PartialEq<Rc<Output>> for Head {
    fn eq(&self, rhs: &Rc<Output>) -> bool {
        self.output().and_then(|lhs| Some(lhs == *rhs)).is_some()
    }
}

