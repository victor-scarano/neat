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

/// Provides a [`FeedForward`] and [`Recurrent`] neural network implementations of the [`Genome`] trait (also defined
/// in this module), as well as the [`Conn`] and [`Node`] genes used to represent them.
pub(crate) mod genome;

pub(crate) mod population;

pub use genome::{FeedForward, Recurrent};
pub use population::{Config, Population};

#[cfg(test)]
mod tests {
	use crate::{*, genome::Genome, population::Innov};

	#[test]
	fn it_works() {
		let mut rng = rand::thread_rng();
		let innov = Innov::default();
		let config = Config;

		let a = FeedForward::<3, 1>::minimal(&mut rng, &innov, &config);
		// a.mutate_split_conn(&mut rng, &innov, &config);
		dbg!(&a);

		// TODO: Fix bug where there are two input and output nodes in child genome.
		// NOTE: Introduced by change at the end of [`Genome::split_conn`].
	}
}
