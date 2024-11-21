use super::*;
use core::fmt;

#[derive(Eq, Clone, Debug, Hash, PartialEq)]
pub enum Head {
    Hidden(Hidden),
    Output(Output),
}

impl Head {
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

impl From<Hidden> for Head {
    fn from(value: Hidden) -> Self {
        Self::Hidden(value)
    }
}

impl From<Output> for Head {
    fn from(value: Output) -> Self {
        Self::Output(value)
    }
}

impl PartialEq<Hidden> for Head {
    fn eq(&self, rhs: &Hidden) -> bool {
        self.hidden().and_then(|lhs| Some(*lhs == *rhs)).is_some()
    }
}

impl PartialEq<Output> for Head {
    fn eq(&self, rhs: &Output) -> bool {
        self.output().and_then(|lhs| Some(*lhs == *rhs)).is_some()
    }
}

impl PartialEq<Tail> for Head {
    fn eq(&self, other: &Tail) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Tail::Hidden(rhs)) => lhs == rhs,
            _ => false
        }
    }
}

