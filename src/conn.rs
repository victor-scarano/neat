use crate::{Innov, Config, node::{ConnInput, ConnOutput, Hidden, Input, Node, Output}};
use std::{cmp::Ordering, fmt, hash, iter, sync::{Arc, RwLock}};
use rand::Rng;

pub(crate) struct Conn {
	input: RwLock<Arc<dyn ConnInput>>,
	output: RwLock<Arc<dyn ConnOutput>>,
	weight: RwLock<f32>,
	enabled: RwLock<bool>,
	innov: u32,
}

impl Conn {
	pub fn new(input: Arc<dyn ConnInput>, output: Arc<dyn ConnOutput>, innov: Arc<Innov>, config: Arc<Config>) -> Self {
		Self {
			input: RwLock::new(input.clone()),
			output: RwLock::new(output.clone()),
			weight: RwLock::new(f32::MAX),
			enabled: RwLock::new(true),
            innov: innov.new_conn_innovation(input, output),
		}
	}

	pub fn input(&self) -> Arc<dyn ConnInput> {
	    self.input.read().unwrap().clone()
	}

	pub fn output(&self) -> Arc<dyn ConnOutput> {
	    self.output.read().unwrap().clone()
	}

	pub fn set_input(&self, f: impl Fn(Arc<dyn ConnInput>) -> Arc<dyn ConnInput>) {
		*self.input.write().unwrap() = f(self.input());
	}

	pub fn set_output(&self, f: impl Fn(Arc<dyn ConnOutput>) -> Arc<dyn ConnOutput>) {
		*self.output.write().unwrap() = f(self.output());
	}

	pub fn weight(&self) -> f32 {
		*self.weight.read().unwrap()
	}

	pub fn enabled(&self) -> bool {
		*self.enabled.read().unwrap()
	}

	pub fn disable(&self) {
		*self.enabled.write().unwrap() = false;
	}

	pub fn innovation(&self) -> u32 {
		self.innov
	}

	pub fn perturbe_weight(&self, rng: &mut impl Rng) {
        todo!();
	}

	pub fn replace_weight(&self, rng: &mut impl Rng) {
        todo!();
	}

	pub fn nodes(&self) -> impl Iterator<Item = Arc<dyn Node>> {
        [self.input() as Arc<dyn Node>, self.output() as Arc<dyn Node>].into_iter()
	}
}

impl Eq for Conn {}

impl fmt::Debug for Conn {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Conn")
			.field("addr", &format_args!("{:?}", self as *const Self))
			.field("input", &format_args!("{:p}", self.input()))
			.field("output", &format_args!("{:p}", self.output()))
			.field("weight", &self.weight)
			.field("enabled", &self.enabled())
			.field("innov", &self.innov)
			.finish()
	}
}

impl hash::Hash for Conn {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		Arc::as_ptr(&self.input()).hash(state);
		Arc::as_ptr(&self.output()).hash(state);
	}
}

impl Ord for Conn {
	fn cmp(&self, other: &Self) -> Ordering {
		self.innov.cmp(&other.innov)
	}
}

impl PartialEq for Conn {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.input(), &other.input()) && Arc::ptr_eq(&self.output(), &other.output())
	}
}

impl PartialOrd for Conn {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

