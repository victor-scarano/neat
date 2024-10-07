use crate::node::*;
use std::{cmp::Ordering, ptr::NonNull};

pub enum UnsafeTrailing {
    Hidden(NonNull<Hidden>),
    Output(NonNull<Output>),
}

impl<'a> From<Trailing<'a>> for UnsafeTrailing {
    fn from(value: Trailing<'a>) -> Self {
        match value {
            Trailing::Hidden(hidden) => UnsafeTrailing::Hidden(NonNull::from(hidden)),
            Trailing::Output(output) => UnsafeTrailing::Output(NonNull::from(output)),
        }
    }
}

#[derive(Eq, Copy, Clone, Debug, PartialEq)]
pub enum Trailing<'a> {
    Hidden(&'a Hidden),
    Output(&'a Output),
}

impl<'a> Trailing<'a> {
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

impl<'a> Node for Trailing<'a> {
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

impl<'a> Trailingable for Trailing<'a> {
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

impl<'a> From<&UnsafeTrailing> for Trailing<'a> {
    fn from(value: &UnsafeTrailing) -> Self {
        match value {
            UnsafeTrailing::Hidden(hidden) => Trailing::Hidden(unsafe { hidden.as_ref() }),
            UnsafeTrailing::Output(output) => Trailing::Output(unsafe { output.as_ref() }),
        }
    }
}

impl<'a> From<&'a Hidden> for Trailing<'a> {
    fn from(value: &'a Hidden) -> Self {
        Self::Hidden(value)
    }
}

impl<'a> From<&'a Output> for Trailing<'a> {
    fn from(value: &'a Output) -> Self {
        Self::Output(value)
    }
}

impl Ord for Trailing<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!();
    }
}

impl<'a> PartialEq<Leading<'a>> for Trailing<'a> {
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
