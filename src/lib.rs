#![feature(result_flattening, trait_upcasting)]
#![warn(clippy::all)]
#![allow(dead_code, clippy::from_over_into, clippy::mutable_key_type, unused_imports, unused_mut, unused_variables)]

pub(crate) mod activation;
mod config;
mod conn;
pub(crate) mod genome;
mod innov;
pub(crate) mod node;
mod population;

pub use activation::Activation;
pub use config::Config;
pub(crate) use conn::Conn;
pub(crate) use genome::Genome;
pub(crate) use innov::Innov;
pub use population::Population;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::{num::NonZero, sync::Arc};

    #[test]
    fn it_works() {
        let mut rng = rand::thread_rng();
        let innov = Arc::new(Innov::default());
        let config = Arc::new(Config::new(
            NonZero::new(1).unwrap(),
            NonZero::new(3).unwrap(),
            NonZero::new(1).unwrap(),
        ));
        let mut genome = Genome::new(&mut rng, innov.clone(), config.clone());
    }
}

