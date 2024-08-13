//! An implementation of the NEAT algorithm (Neuro Evolution of Augmenting Topologies) in Rust.
//!
//! This implementation of NEAT is based off Kenith Stanley's
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

#![allow(unused_variables, dead_code, clippy::mutable_key_type)]
// #![warn(missing_docs)]

pub(crate) mod conn;
pub(crate) mod node;
pub(crate) mod genome;

pub use genome::FeedForward;

#[cfg(test)]
mod tests {
    use crate::genome::{FeedForward, Genome};

    #[test]
    fn it_works() {
        let mut rng = rand::thread_rng();

        let mut a = FeedForward::<1, 1>::minimal();
        let input = a.iter_input().nth(0).unwrap();
        let output = a.iter_output().nth(0).unwrap();
        let old_conn = a.add_conn(input, output, 0.5, 0);
        a.split_conn(old_conn, 1, 2);
        a.set_fitness(10.0);
        dbg!(&a);

        let mut b = FeedForward::<1, 1>::minimal();
        let input = b.iter_input().nth(0).unwrap();
        let output = b.iter_output().nth(0).unwrap();
        let old_conn = b.add_conn(input, output, 0.5, 0);
        b.split_conn(old_conn, 1, 2);
        b.set_fitness(10.0);
        dbg!(&b);

        let c = FeedForward::crossover(a, b, &mut rng);
        dbg!(&c);

        // TODO: Fix bug where there are two input and output nodes in child genome.
        // NOTE: Introduced by change at the end of `Genome::split_conn`
    }
}
