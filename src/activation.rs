use std::{any::Any, fmt, f32::consts::E, ops::Deref, hash, sync::Arc};

#[derive(Clone)]
pub struct Activation(Arc<(dyn Fn(f32) -> f32 + Send + Sync)>);

impl Activation {
    pub(crate) fn call(&self, x: f32) -> f32 {
        self.0(x)
    }
}

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
        Activation(Arc::new(|x| x))
    }
}

pub struct Sigmoid;

impl Into<Activation> for Sigmoid {
    fn into(self) -> Activation {
        Activation(Arc::new(|x| 1.0 / (1.0 + E.powf(-x))))
    }
}

