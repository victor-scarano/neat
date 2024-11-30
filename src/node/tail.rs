extern crate alloc;
use crate::node::*;
use core::fmt;
use alloc::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Tail {
    Input(Rc<Input, Bump>),
    Hidden(Rc<Hidden, Bump>),
}

impl Tail {
    pub fn input(&self) -> Option<Rc<Input, Bump>> {
        match self {
            Self::Input(input) => Some(input.clone()),
            Self::Hidden(_) => None,
        }
    }

    pub fn hidden(&self) -> Option<Rc<Hidden, Bump>> {
        match self {
            Self::Input(_) => None,
            Self::Hidden(hidden) => Some(hidden.clone()),
        }
    }

    pub fn innov(&self) -> usize {
        match self {
            Self::Input(input) => input.innov(),
            Self::Hidden(hidden) => hidden.innov(),
        }
    }

    pub fn allocator(&self) -> Bump {
        match self {
            Self::Input(ref input) => Rc::allocator(input).clone(),
            Self::Hidden(ref hidden) => Rc::allocator(hidden).clone(),
        }
    }
}

impl Node for Tail {
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

impl fmt::Pointer for Tail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(ref input) => fmt::Pointer::fmt(&input, f),
            Self::Hidden(ref hidden) => fmt::Pointer::fmt(&hidden, f),
        }
    }
}

impl From<Rc<Hidden, Bump>> for Tail {
    fn from(value: Rc<Hidden, Bump>) -> Self {
        Self::Hidden(value)
    }
}

impl PartialEq<Rc<Input, Bump>> for Tail {
    fn eq(&self, rhs: &Rc<Input, Bump>) -> bool {
        self.input().map(|lhs| lhs == *rhs).is_some()
    }
}

impl PartialEq<Rc<Hidden, Bump>> for Tail {
    fn eq(&self, rhs: &Rc<Hidden, Bump>) -> bool {
        self.hidden().map(|lhs| lhs == *rhs).is_some()
    }
}

impl PartialEq<Head> for Tail {
    fn eq(&self, other: &Head) -> bool {
        // are we supposed to check for allocation equality or partial eq equality?
        self.hidden().and_then(|lhs| other.hidden().map(|rhs| Rc::ptr_eq(&lhs, &rhs))).is_some()
    }
}

