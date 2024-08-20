use crate::Node;
use std::{cell::Cell, cmp::Ordering, fmt, hash, rc::Rc};
use rand::Rng;

#[derive(Default)]
pub(crate) struct Connection {
	input: Cell<Rc<Node>>,
	output: Cell<Rc<Node>>,
	weight: Cell<f32>,
	enabled: Cell<bool>,
	innovation: u32,
}

impl Connection {
	pub fn new(input: Rc<Node>, output: Rc<Node>, weight: f32, innov: u32) -> Self {
		Self {
			input: Cell::new(input),
			output: Cell::new(output),
			weight: Cell::new(weight),
			enabled: Cell::new(true),
			innovation: innov,
		}
	}

	pub fn input(&self) -> Rc<Node> {
		let node = self.input.take();
		self.input.set(node.clone());
		node.clone()
	}

	pub fn output(&self) -> Rc<Node> {
		let node = self.output.take();
		self.output.set(node.clone());
		node.clone()
	}

	pub fn set_input(&self, f: impl Fn(Rc<Node>) -> Rc<Node>) {
		self.input.set(f(self.input()));
	}

	pub fn set_output(&self, f: impl Fn(Rc<Node>) -> Rc<Node>) {
		self.output.set(f(self.output()));
	}

	pub fn weight(&self) -> f32 {
		self.weight.get()
	}

	pub fn enabled(&self) -> bool {
		self.enabled.get()
	}

	pub fn disable(&self) {
		self.enabled.set(false);
	}

	pub fn innovation(&self) -> u32 {
		self.innovation
	}

	pub fn perturbe_weight(&self, rng: &mut impl Rng) {
		let weight = self.weight.take();
		self.weight.set(weight + rng.gen::<f32>());
	}

	pub fn replace_weight(&self, rng: &mut impl Rng) {
		self.weight.set(rng.gen())
	}

	pub fn nodes(&self) -> impl Iterator<Item = Rc<Node>> {
		[self.input(), self.output()].into_iter()
	}
}

impl Clone for Connection {
	fn clone(&self) -> Self {
		Self {
			input: Cell::new(self.input()),
			output: Cell::new(self.output()),
			weight: self.weight.clone(),
			enabled: self.enabled.clone(),
			innovation: self.innovation,
		}
	}
}

impl Eq for Connection {}

impl fmt::Debug for Connection {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Conn")
			.field("addr", &format_args!("{:?}", self as *const Self))
			.field("input", &format_args!("{:p}", self.input()))
			.field("output", &format_args!("{:p}", self.output()))
			.field("weight", &self.weight)
			.field("enabled", &self.enabled())
			.field("innov", &self.innovation)
			.finish()
	}
}

impl hash::Hash for Connection {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		Rc::as_ptr(&self.input()).hash(state);
		Rc::as_ptr(&self.output()).hash(state);
	}
}

impl Ord for Connection {
	fn cmp(&self, other: &Self) -> Ordering {
		self.innovation.cmp(&other.innovation)
	}
}

impl PartialEq for Connection {
	fn eq(&self, other: &Self) -> bool {
		Rc::ptr_eq(&self.input(), &other.input()) && Rc::ptr_eq(&self.output(), &other.output())
	}
}

impl PartialOrd for Connection {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

