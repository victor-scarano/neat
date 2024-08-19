#![warn(clippy::all)]

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

pub(crate) mod config;

/// Provides a [`FeedForward`] and [`Recurrent`] neural network implementations of the [`Genome`] trait (also defined
/// in this module), as well as the [`Conn`] and [`Node`] genes used to represent them.
pub(crate) mod genome;

pub(crate) mod innov;

pub(crate) mod population;

pub use activation::{Activation, activations};
pub use config::Config;
pub use genome::{FeedForward, Recurrent};
pub(crate) use innov::Innov;
pub use population::Population;

#[cfg(test)]
mod tests {
	use crate::{*, genome::Genome};
	use std::num::NonZeroUsize;
	use rand::{seq::IteratorRandom, thread_rng, Rng};

	#[test]
	fn it_workls() {
		let mut rng = thread_rng();

		let innov = Innov::default();

		let config = Config {
			num_inputs: NonZeroUsize::new(3).unwrap(),
			num_outputs: NonZeroUsize::new(1).unwrap(),
		};

		let mut genome = FeedForward::minimal(&mut rng, &innov, &config);

		let some_conn = genome.iter_conns().choose(&mut rng).unwrap();
		let new_conn = genome.add_conn(some_conn.input(), some_conn.output(), rng.gen(), &innov);
		assert_eq!(some_conn.innov(), new_conn.innov());

		let (some_conn, _) = genome.split_conn(some_conn, &innov);
		let new_conn = genome.add_conn(some_conn.input(), some_conn.output(), rng.gen(), &innov);
		assert_eq!(some_conn.innov(), new_conn.innov());
	}
}
