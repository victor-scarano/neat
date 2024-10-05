use crate::node::*;
use std::cmp::Ordering;

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum Trailing<'g> {
    Hidden(&'g Hidden),
    Output(&'g Output),
}

impl<'g> Trailing<'g> {
    pub const fn hidden(&self) -> Option<&Hidden> {
        match self {
            Self::Hidden(hidden) => Some(hidden),
            Self::Output(_) => None,
        }
    }

    pub const fn output(&self) -> Option<&Output> {
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

impl<'g> Node for Trailing<'g> {
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

impl<'g> Trailingable for Trailing<'g> {
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

impl<'g> From<&'g Hidden> for Trailing<'g> {
    fn from(value: &'g Hidden) -> Self {
        Self::Hidden(value)
    }
}

impl<'g> From<&'g Output> for Trailing<'g> {
    fn from(value: &'g Output) -> Self {
        Self::Output(value)
    }
}

impl Ord for Trailing<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!();
    }
}

impl<'g> PartialEq<Leading<'g>> for Trailing<'g> {
    fn eq(&self, other: &Leading) -> bool {
        match (self, other) {
            (Self::Hidden(lhs), Leading::Hidden(rhs)) => lhs == rhs,
            _ => false
        }
    }
}

impl PartialOrd for Trailing<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
