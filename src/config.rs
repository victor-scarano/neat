#[allow(dead_code)]

use std::{iter, num::NonZero};
use crate::{Activation, activations};
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Config {
    /// The number of individuals in each generation.
    pop_size: NonZero<u32>,

    /// A list of the activation functions that may be used by nodes.
    /// This defaults to [`Sigmoid`](activations::Sigmoid).
    activations: Vec<Box<dyn Activation>>,
    /// The default activation function assigned to new nodes.
    /// If [`None`] is given, one of the activations in [`Self::activation_options`] will be chosen at random.
    default_activation: Box<dyn Activation>,
    /// The probability that mutation will replace the node's activation function with a randomly determined member of
    /// the activations in [`Self::activation_options`]. Valid values are in [0.0, 1.0].
    activation_mutate_rate: f32,

	num_inputs: NonZero<usize>,
    num_hidden: u32,
	num_outputs: NonZero<usize>,
}

impl Config {
    pub fn new(pop_size: NonZero<u32>, num_inputs: NonZero<usize>, num_outputs: NonZero<usize>) -> Self {
        Self {
            pop_size,

            activations: iter::once(Box::new(activations::Sigmoid) as Box<dyn Activation>).collect(),
            default_activation: Box::new(activations::Sigmoid),
            activation_mutate_rate: 0.5,

            num_inputs,
            num_hidden: 0,
            num_outputs,
        }
    }

    pub fn with_activations(&mut self, activations: impl IntoIterator<Item = Box<dyn Activation>>) -> &mut Self {
        self.activations = activations.into_iter().collect();
        self.default_activation = self.activations.choose(&mut rand::thread_rng()).unwrap();
        self
    }

    pub fn insert_activation(&mut self, activation: Box<dyn Activation>) -> &mut Self {
        self.activations.push(activation);
        self
    }

    pub fn pop_size(&self) -> u32 { self.pop_size.get() }

    pub fn num_inputs(&self) -> usize { self.num_inputs.get() }

    pub fn num_outputs(&self) -> usize { self.num_outputs.get() }
}
