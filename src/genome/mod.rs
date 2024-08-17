#![allow(dead_code, clippy::mutable_key_type, unused_variables)]

use rand::Rng;

mod conn;
mod feedforward;
mod node;
mod recurrent;

pub(crate) use conn::Conn;
pub use feedforward::FeedForward;
pub(crate) use node::Node;
pub use recurrent::Recurrent;

/// Provides an interface generic over a variety of neural network structures that is interoperable with the
/// [`Population`](crate::population::Population).
pub(crate) trait Genome<const I: usize, const O: usize> {
	type Config;
	type Innov;

	/// Constructs a 'minimal' genome with no hidden nodes. All input nodes are connected to all output nodes.
	fn minimal(rng: &mut impl Rng, innov: &Self::Innov, config: &Self::Config) -> Self;

	/// A single new connection with a random weight is added connecting two previously unconnected nodes.
	fn mutate_add_conn(&mut self, rng: &mut impl Rng, innov: &Self::Innov, config: &Self::Config);

	/// An existing connection is split and the new node placed where the old connection used to be. The old connection
	/// is disabled and two new connections are added to the genome. The new connection leading into the new node
	/// receives a weight of 1.0, and the new connection leading out receives the same weight as the old connection.
	///
	/// # Note
	///
	/// In Stanley's NEAT paper, this is referred to as the 'add node' mutation. I refer to it as the 'add connection'
	/// mutation in my implementation because it is easier for me to understand that way.
	fn mutate_split_conn(&mut self, rng: &mut impl Rng, innov: &Self::Innov, config: &Self::Config);

	/// Mutates a random connection's weight.
	fn mutate_conn_weight(&mut self, rng: &mut impl Rng, config: &Self::Config);

	/// Activates the genome, taking an array of its inputs and returns an array of outputs.
	fn activate(&self, inputs: [f32; I], config: &Self::Config) -> [f32; O];

	/// Sets the genome's fitness.
	fn set_fitness(&mut self, fitness: f32, config: &Self::Config);

	/// Computes the compatibility distance between two genomes.
	fn compat_dist(lhs: &Self, rhs: &Self, config: &Self::Config) -> f32;

	/// Consumes two parent genomes and returns a child genome.
	fn crossover(lhs: Self, rhs: Self, rng: &mut impl Rng, config: &Self::Config) -> Self;
}