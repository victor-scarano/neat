use std::num::NonZeroUsize;

pub struct Config {
	pub num_inputs: NonZeroUsize,
	pub num_outputs: NonZeroUsize,
}

impl Config {
	pub(crate) fn num_inputs(&self) -> usize {
		self.num_inputs.get()
	}

	pub(crate) fn num_outputs(&self) -> usize {
		self.num_outputs.get()
	}
}