use core::fmt;
use std::{any::Any, f32::consts::E, hash, rc::Rc};

#[derive(Clone)]
pub struct Activation(Rc<dyn Fn(f32) -> f32>);

impl fmt::Debug for Activation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Activation").finish()
    }
}

impl Default for Activation {
    fn default() -> Self {
        Sigmoid.into()
    }
}

impl Eq for Activation {}

impl hash::Hash for Activation {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.as_ref().type_id().hash(state);
    }
}

impl PartialEq for Activation {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref().type_id() == other.0.as_ref().type_id()
    }
}

pub struct Identity;

impl Into<Activation> for Identity {
    fn into(self) -> Activation {
        Activation(Rc::new(|x| x))
    }
}

pub struct Sigmoid;

impl Into<Activation> for Sigmoid {
    fn into(self) -> Activation {
        Activation(Rc::new(|x| 1.0 / (1.0 + E.powf(-x))))
    }
}

