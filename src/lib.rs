#![warn(clippy::all)]
#![allow(dead_code, unused_variables)]

//! An implementation of the NEAT algorithm (Neuro Evolution of Augmenting Topologies) in Rust.
//!
//! This implementation is based off Kenith Stanley's
//! [paper](https://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf).
//!
//! # Features
//!
//! - [ ] Feed forward neural network implementation
//! - [ ] Recurrent neural network implementation
//! - [ ] Serde support for individual genomes
//! - [ ] Serde support for full populations
//! - [ ] Evolution of multiple populations concurrently
//! - [ ] Concurrent evolution of a single population

pub(crate) mod activation;
mod config;
mod connection;
mod genome;
mod innovation;
mod node;
mod population;

pub use activation::Activation;
pub use config::Config;
pub(crate) use connection::Connection;
pub use genome::Genome;
pub(crate) use innovation::Innovation;
pub(crate) use node::Node;
pub use population::Population;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

