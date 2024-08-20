use crate::Activation;
use std::{cell::OnceCell, num::NonZero};
use rand::seq::SliceRandom;

pub struct Config {
    /// The number of individuals in each generation.
    pop_size: usize,

    /// A list of the activation functions that may be used by nodes.
    /// This defaults to [`Sigmoid`](crate::activations::Sigmoid).
    activations: Vec<Activation>,
    /// The default activation function assigned to new nodes.
    /// If [`None`] is given, then one of the activations in [`Self::activations`] will be chosen at random.
    default_activation: OnceCell<Activation>,
    /// The probability that mutation will replace the node's activation function with a randomly determined member of
    /// the activations in [`Self::activation_options`].
    activation_mutate_rate: f32,

    // TODO: Aggregation.
    // TODO: Bias.
    
    /// The coefficient for the disjoint and excess gene counts' contribution to the genomic distance.
    compat_disjoint_coeff: f32,
    /// The coefficient for each weight, bias, or response multiplier difference's contribution to the genomic distance
    /// (for homologous nodes or connections). This is also used as the value to add for differences in activation
    /// functions, aggregation functions, or enabled/disabled status.
    compat_weight_coeff: f32,

    /// The probability that mutation will add a connection between existing nodes.
    add_conn_prob: f32,
    /// The probability that mutation will remove an existing connection.
    remove_conn_prob: f32,
    /// Whether or not new connections are enabled by default.
    enabled_default: bool,

	num_inputs: usize,
    num_hidden: usize,
	num_outputs: usize,
}

impl Config {
    pub fn new(pop_size: NonZero<usize>, num_inputs: NonZero<usize>, num_outputs: NonZero<usize>) -> Self {
        Self {
            pop_size: pop_size.get(),
            activations: Vec::new(),
            default_activation: OnceCell::new(),
            activation_mutate_rate: 0.5,
            compat_disjoint_coeff: 0.5,
            compat_weight_coeff: 0.5,
            add_conn_prob: 0.5,
            remove_conn_prob: 0.5,
            enabled_default: true,
            num_inputs: num_inputs.get(),
            num_hidden: 0,
            num_outputs: num_outputs.get(),
        }
    }

    fn update_default_activation(&mut self) {
        let choice = self.activations.choose(&mut rand::thread_rng()).cloned().unwrap();
        let _ = self.default_activation.set(choice);
    }

    pub fn with_activations(mut self, activations: impl IntoIterator<Item = impl Into<Activation>>) -> Self {
        self.activations = activations.into_iter().map(|activation| activation.into()).collect();
        self.update_default_activation();
        self
    }

    pub fn insert_activation(mut self, activation: impl Into<Activation>) -> Self {
        self.activations.push(activation.into());
        self.update_default_activation();
        self
    }

    pub fn with_activation_mutate_rate(mut self, value: f32) -> Self {
        assert!(value >= 0.0);
        assert!(value <= 1.0);
        self.activation_mutate_rate = value;
        self
    }

    pub fn with_compat_disjoint_coeff(mut self, value: f32) -> Self {
        self.compat_disjoint_coeff = value;
        self
    }

    pub fn with_compat_weight_coeff(mut self, value: f32) -> Self {
        self.compat_weight_coeff = value;
        self
    }

    pub fn with_add_conn_prob(mut self, value: f32) -> Self {
        assert!(value >= 0.0);
        assert!(value <= 1.0);
        self.add_conn_prob = value;
        self
    }

    pub fn with_remove_conn_prob(mut self, value: f32) -> Self {
        assert!(value >= 0.0);
        assert!(value <= 1.0);
        self.remove_conn_prob = value;
        self
    }

    pub fn with_enabled_default_disabled(mut self) -> Self {
        self.enabled_default = false;
        self
    }



    pub(crate) fn pop_size(&self) -> usize {
        self.pop_size
    }

    pub(crate) fn activations(&self) -> impl Iterator<Item = &Activation> {
        self.activations.iter()
    }

    pub(crate) fn default_activation(&self) -> &Activation {
        self.default_activation.get().unwrap()
    }

    pub(crate) fn activation_mutate_rate(&self) -> f32 {
        self.activation_mutate_rate
    }

    pub(crate) fn compat_disjoint_coeff(&self) -> f32 {
        self.compat_disjoint_coeff
    }

    pub(crate) fn compat_weight_coeff(&self) -> f32 {
        self.compat_weight_coeff
    }

    pub(crate) fn add_conn_prob(&self) -> f32 {
        self.add_conn_prob
    }

    pub(crate) fn remove_conn_prob(&self) -> f32 {
        self.remove_conn_prob
    }

    pub(crate) fn enabled_default(&self) -> bool {
        self.enabled_default
    }

    pub(crate) fn num_inputs(&self) -> usize {
        self.num_inputs
    }

    pub(crate) fn num_hidden(&self) -> usize {
        self.num_hidden
    }

    pub(crate) fn num_outputs(&self) -> usize {
        self.num_outputs
    }
}

