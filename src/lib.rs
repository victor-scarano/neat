#![warn(clippy::all)]
#![allow(clippy::mutable_key_type, dead_code, unused_variables)]

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

/// Provides an implementation for node genes (also known as neurons), and connection genes that link two node genes.
pub(crate) mod genes;

/// Provides a feedforward and recurrent neural network implementations of the [`Genome`] trait, also defined in this
/// module.
pub(crate) mod genome;

#[allow(clippy::missing_docs_in_private_items)]
pub(crate) mod population;

pub use genome::{Config, FeedForward, Genome};
pub use population::{Innov, Population};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut rng = rand::thread_rng();
        let config = Config {};
        let innov = ();

        let mut a = FeedForward::<1, 1>::minimal(&innov, &config);
        let input = a.iter_input().nth(0).unwrap();
        let output = a.iter_output().nth(0).unwrap();
        let old_conn = a.add_conn(input, output, 0.5, 0);
        a.split_conn(old_conn, 1, 2);
        a.set_fitness(10.0, &config);
        dbg!(&a);

        let mut b = FeedForward::<1, 1>::minimal(&innov, &config);
        let input = b.iter_input().nth(0).unwrap();
        let output = b.iter_output().nth(0).unwrap();
        let old_conn = b.add_conn(input, output, 0.5, 0);
        b.split_conn(old_conn, 1, 2);
        b.set_fitness(10.0, &config);
        dbg!(&b);

        let c = FeedForward::crossover(a, b, &mut rng, &config);
        dbg!(&c);

        // TODO: Fix bug where there are two input and output nodes in child genome.
        // NOTE: Introduced by change at the end of [`Genome::split_conn`].
    }
}
