use std::{f32::consts::E, rc::Rc};

#[derive(Clone)]
pub struct Activation(Rc<dyn Fn(f32) -> f32>);

impl Default for Activation {
    fn default() -> Self {
        Sigmoid.into()
    }
}

pub struct Sigmoid;

impl Into<Activation> for Sigmoid {
    fn into(self) -> Activation {
        Activation(Rc::new(|x| 1.0 / (1.0 + E.powf(-x))))
    }
}

