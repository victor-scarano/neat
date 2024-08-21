use crate::{activation, Activation};
use std::{cell::OnceCell, num::NonZero};
use rand::{seq::SliceRandom, Rng};
use rand_distr::{Distribution, Normal, Uniform};

pub enum BiasType {
    Normal,
    Uniform,
}

impl BiasType {
    fn sample(&self, rng: &mut impl Rng, config: &Config) -> f32 {
        match self {
            Self::Normal => {
                Normal::new(config.default_bias_mean, config.default_bias_std_dev).unwrap().sample(rng)
            },
            Self::Uniform => {
                Uniform::new_inclusive(
                    f32::max(config.bias_min, config.default_bias_mean - (config.default_bias_std_dev * 2.0)),
                    f32::min(config.bias_max, config.default_bias_mean + (config.default_bias_std_dev * 2.0)),
                ).sample(rng)
            }
        }
    }
}

pub struct Config {
    pop_size: usize,

    activations: OnceCell<Vec<Activation>>,
    default_activation: OnceCell<Activation>,
    activation_mutate_rate: f32,

    // TODO: Aggregation.

    default_bias_mean: f32,
    default_bias_std_dev: f32,
    default_bias_type: BiasType,
    bias_max: f32,
    bias_min: f32,
    bias_mutate_power: f32,
    bias_mutate_rate: f32,
    bias_replace_rate: f32,

    compat_disjoint_coeff: f32,
    compat_weight_coeff: f32,

    add_conn_prob: f32,
    remove_conn_prob: f32,
    enabled_default: bool,

	num_inputs: usize,
    num_hidden: usize,
	num_outputs: usize,
}

impl Config {
    pub fn new(pop_size: NonZero<usize>, num_inputs: NonZero<usize>, num_outputs: NonZero<usize>) -> Self {
        Self {
            pop_size: pop_size.get(),
            activations: OnceCell::new(),
            default_activation: OnceCell::new(),
            activation_mutate_rate: 0.5,
            default_bias_mean: 0.5,
            default_bias_std_dev: 0.5,
            default_bias_type: BiasType::Normal,
            bias_max: 0.5,
            bias_min: 0.5,
            bias_mutate_power: 0.5,
            bias_mutate_rate: 0.5,
            bias_replace_rate: 0.5,
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

    fn update_default_activation(&mut self, rng: &mut impl Rng) {
        let choice = self.activations.get_or_init(|| vec![activation::Sigmoid.into()]).choose(rng).cloned().unwrap();
        let _ = self.default_activation.set(choice);
    }

    pub fn with_activations(mut self, rng: &mut impl Rng, activations: impl IntoIterator<Item = impl Into<Activation>>) -> Self {
        let _ = self.activations.set(activations.into_iter().map(|activation| activation.into()).collect());
        self.update_default_activation(rng);
        self
    }

    pub fn insert_activation(mut self, rng: &mut impl Rng, activation: impl Into<Activation>) -> Self {
        let _ = self.activations.get_or_init(|| Vec::new());
        self.activations.get_mut().unwrap().push(activation.into());
        self.update_default_activation(rng);
        self
    }

    pub fn with_activation_mutate_rate(mut self, value: f32) -> Self {
        assert!(value >= 0.0);
        assert!(value <= 1.0);
        self.activation_mutate_rate = value;
        self
    }

    pub fn with_default_bias_mean(mut self, value: f32) -> Self {
        self.default_bias_mean = value;
        self
    }

    pub fn with_default_bias_std_dev(mut self, value: f32) -> Self {
        self.default_bias_std_dev = value;
        self
    }
    
    pub fn with_default_bias_type(mut self, value: BiasType) -> Self {
        self.default_bias_type = value;
        self
    }

    pub fn with_bias_max(mut self, value: f32) -> Self {
        self.bias_max = value;
        self
    }

    pub fn with_bias_min(mut self, value: f32) -> Self {
        self.bias_min = value;
        self
    }

    pub fn with_bias_mutate_power(mut self, value: f32) -> Self {
        self.bias_mutate_power = value;
        self
    }

    pub fn with_bias_mutate_rate(mut self, value: f32) -> Self {
        self.bias_mutate_rate = value;
        self
    }

    pub fn with_bias_replace_rate(mut self, value: f32) -> Self {
        self.bias_replace_rate = value;
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
        self.activations.get_or_init(|| vec![activation::Sigmoid.into()]).iter()
    }

    pub(crate) fn default_activation(&self) -> Activation {
        self.default_activation.get().cloned().unwrap()
    }

    pub(crate) fn activation_mutate_rate(&self) -> f32 {
        self.activation_mutate_rate
    }

    pub(crate) fn new_node_bias(&self, rng: &mut impl Rng) -> f32 {
        self.default_bias_type.sample(rng, self)
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

