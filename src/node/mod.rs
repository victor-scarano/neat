mod accum;
mod input;
mod hidden;
mod output;
mod head;
mod tail;

use core::{fmt, pin::Pin};
pub use head::*;
pub use hidden::*;
pub use input::*;
pub use output::*;
pub use tail::*;

pub trait Node {
    fn layer(&self) -> usize;
    fn bias(&self) -> f32;
    fn innov(&self) -> usize;
    fn update_layer(&self, layer: usize);
    fn activate(&self, x: f32) -> f32;
    fn response(&self) -> f32;
    fn aggregator(&self) -> fn(&[f32]) -> f32;
}

// kinda ugly and not very ideal, but for now its needed for graphviz (without using dynamic dispatch)
// maybe there is a better way we can manage nodes in general to avoid all of these types.
// if preferably call this type node, but then the node trait would be conflicting
#[derive(Clone)]
pub enum AnyNode<'g> {
    Input(Pin<&'g Input>),
    Hidden(Pin<&'g Hidden>),
    Output(Pin<&'g Output>),
}

impl From<Tail<'_>> for AnyNode<'_> {
    fn from(value: Tail) -> Self {
        match value {
            Tail::Input(input) => Self::Input(input),
            Tail::Hidden(hidden) => Self::Hidden(hidden),
        }
    }
}

impl From<Head<'_>> for AnyNode<'_> {
    fn from(value: Head) -> Self {
        match value {
            Head::Hidden(hidden) => Self::Hidden(hidden),
            Head::Output(output) => Self::Output(output),
        }
    }
}

impl From<&Pin<&Input>> for AnyNode<'_> {
    fn from(value: &Pin<&Input>) -> Self {
        Self::Input(value)
    }
}

impl From<&Pin<&Hidden>> for AnyNode<'_> {
    fn from(value: &Pin<&Hidden>) -> Self {
        Self::Hidden(value)
    }
}

impl From<&Pin<&Output>> for AnyNode<'_> {
    fn from(value: &Pin<&Output>) -> Self {
        Self::Output(value)
    }
}

impl Node for AnyNode<'_> {
    fn bias(&self) -> f32 {
        todo!()
    }

    fn layer(&self) -> usize {
        todo!()
    }

    fn innov(&self) -> usize {
        match self {
            Self::Input(ref input) => input.innov,
            Self::Hidden(ref hidden) => hidden.innov,
            Self::Output(ref output) => output.innov,
        }
    }
    
    fn activate(&self, x: f32) -> f32 {
        todo!()
    }

    fn response(&self) -> f32 {
        todo!()
    }

    fn aggregator(&self) -> fn(&[f32]) -> f32 {
        todo!()
    }

    fn update_layer(&self, layer: usize) {
        todo!()
    }
}

impl fmt::Pointer for AnyNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(ref input) => fmt::Pointer::fmt(input, f),
            Self::Hidden(ref hidden) => fmt::Pointer::fmt(hidden, f),
            Self::Output(ref output) => fmt::Pointer::fmt(output, f),
        }
    }
}

