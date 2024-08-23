use crate::node::{ConnInput, ConnOutput, Hidden, Input, Node, Output};
use std::{borrow::Borrow, cell::{Cell, RefCell}, cmp::Ordering, fmt, hash, iter, rc::Rc};
use rand::Rng;

pub(crate) struct Conn {
	input: RefCell<Rc<dyn ConnInput>>,
	output: RefCell<Rc<dyn ConnOutput>>,
	weight: Cell<f32>,
	enabled: Cell<bool>,
	innovation: u32,
}

impl Conn {
	pub fn new(input: Rc<dyn ConnInput>, output: Rc<dyn ConnOutput>, weight: f32, innov: u32) -> Self {
		Self {
			input: RefCell::new(input),
			output: RefCell::new(output),
			weight: Cell::new(weight),
			enabled: Cell::new(true),
			innovation: innov,
		}
	}

	pub fn input(&self) -> Rc<dyn ConnInput> {
		Rc::clone(&self.input.borrow())
	}

	pub fn output(&self) -> Rc<dyn ConnOutput> {
        Rc::clone(&self.output.borrow())
	}

	pub fn set_input(&self, f: impl Fn(Rc<dyn ConnInput>) -> Rc<dyn ConnInput>) {
		self.input.replace(f(self.input()));
	}

	pub fn set_output(&self, f: impl Fn(Rc<dyn ConnOutput>) -> Rc<dyn ConnOutput>) {
		self.output.replace(f(self.output()));
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
		// let weight = self.weight.take();
		// self.weight.set(weight + rng.gen::<f32>());
        todo!();
	}

	pub fn replace_weight(&self, rng: &mut impl Rng) {
		// self.weight.set(rng.gen())
        todo!();
	}

	pub fn nodes(&self) -> impl Iterator<Item = Rc<dyn Node>> {
        [self.input() as Rc<dyn Node>, self.output() as Rc<dyn Node>].into_iter()
	}
}

impl Clone for Conn {
	fn clone(&self) -> Self {
		Self {
			input: RefCell::new(self.input()),
			output: RefCell::new(self.output()),
			weight: self.weight.clone(),
			enabled: self.enabled.clone(),
			innovation: self.innovation,
		}
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
			.field("innov", &self.innovation)
			.finish()
	}
}

impl hash::Hash for Conn {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		Rc::as_ptr(&self.input()).hash(state);
		Rc::as_ptr(&self.output()).hash(state);
	}
}

impl Ord for Conn {
	fn cmp(&self, other: &Self) -> Ordering {
		self.innovation.cmp(&other.innovation)
	}
}

impl PartialEq for Conn {
	fn eq(&self, other: &Self) -> bool {
		Rc::ptr_eq(&self.input(), &other.input()) && Rc::ptr_eq(&self.output(), &other.output())
	}
}

impl PartialOrd for Conn {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

