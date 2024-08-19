#[allow(dead_code)]

use std::fmt::Debug;

pub trait Activation: Debug {
    fn activate(&self, x: f32) -> f32;
}

pub mod activations {
    use super::Activation;
    use std::f32::consts::E;

    #[derive(Debug)]
    pub struct Sigmoid;
    impl Activation for Sigmoid {
        fn activate(&self, x: f32) -> f32 {
            1.0 / (1.0 + E.powf(-x))
        }
    }
}
