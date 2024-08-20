use rand::Rng;
use crate::Node;
use std::{cell::Cell, cmp::Ordering, fmt, hash, rc::Rc};

/// A connection between two [`Node`]s (also known as neurons) within the [`Genome`](crate::Genome).
#[derive(Default)]
pub(crate) struct Connection {
	/// The [`Node`] leading into the connection.
	///
	/// We wrap [`Rc<Node>`] in a [`Cell`] to be able to safely [`mem::swap`](std::mem::swap) the [`Rc`] to point to a
	/// different [`Node`] without needing a `&mut self`. See [`Self::set_input`] for usage.
	input: Cell<Rc<Node>>,

	/// The [`Node`] leading out of the connection.
	///
	/// We wrap [`Rc<Node>`] in a [`Cell`] to be able to safely [`mem::swap`](std::mem::swap) the [`Rc`] to point to a
	/// different [`Node`] without needing a `&mut self`. See [`Self::set_output`] for usage.
	output: Cell<Rc<Node>>,

	/// The weight of the connection.
	///
	/// We wrap the [`f32`] in a [`Cell`] to provide interior mutability of the weight, since most method calls to
	/// `Self` are handled through an [`Rc`].
	weight: Cell<f32>,

	/// The enabled status of the connection.
	///
	/// We wrap the [`bool`] in a [`Cell`] to provide interior mutability of the enabled status, since most method
	/// calls to `Self` are handled through an [`Rc`].
	enabled: Cell<bool>,

	/// The innovation of the connection.
	innovation: u32,
}

impl Connection {
	/// Constructs a new connection.
	///
	/// The input and output parameters to this function are [`Rc<Node>`]s instead of [`Node`]s because it (somewhat)
	/// ensures that they have been already been inserted into the genome.
	pub fn new(input: Rc<Node>, output: Rc<Node>, weight: f32, innov: u32) -> Self {
		Self {
			input: Cell::new(input),
			output: Cell::new(output),
			weight: Cell::new(weight),
			enabled: Cell::new(true),
			innovation: innov,
		}
	}

	/// Returns the [`Node`] feeding into the connection.
	pub fn input(&self) -> Rc<Node> {
		let node = self.input.take();
		self.input.set(node.clone());
		node.clone()
	}

	/// Returns the [`Node`] feeding out of the connection.
	pub fn output(&self) -> Rc<Node> {
		let node = self.output.take();
		self.output.set(node.clone());
		node.clone()
	}

	/// Sets the input node of the connection based on a predicate where the predicate gives the current input node,
	/// and the expected return value is the new input node that will replace the current one.
	///
	/// This function is useful when you have a map where the key is the current node and the value is a new node, and
	/// you want to update the connection's input node based on the key.
	///
	/// # Example:
	/// 
	/// ```ignore
	/// let map: Map<Rc<Node>, Rc<Node>> = ...
	/// 
	/// for conn in map.iter_keys() {
	///     conn.set_input(|node| map.get(&node).cloned().unwrap());
	/// }
	/// ```
	pub fn set_input(&self, f: impl Fn(Rc<Node>) -> Rc<Node>) {
		self.input.set(f(self.input()));
	}

	/// Sets the output node of the connection based on a predicate where the predicate gives the current output node,
	/// and the expected return value is the new output node that will replace the current one.
	///
	/// This function is useful when you have a map where the key is the current node and the value is a new node, and
	/// you want to update the connection's output node based on the key.
	///
	/// # Example:
	/// 
	/// ```ignore
	/// let map: Map<Rc<Node>, Rc<Node>> = ...
	/// 
	/// for conn in map.iter_keys() {
	///     conn.set_output(|node| map.get(&node).cloned().unwrap());
	/// }
	/// ```
	pub fn set_output(&self, f: impl Fn(Rc<Node>) -> Rc<Node>) {
		self.output.set(f(self.output()));
	}

	/// Returns the weight of the connection.
	pub fn weight(&self) -> f32 {
		self.weight.get()
	}

	/// Returns the enabled status of the connection.
	pub fn enabled(&self) -> bool {
		self.enabled.get()
	}

	/// Disables the connection.
	pub fn disable(&self) {
		self.enabled.set(false);
	}

    /// Returns the node's innovation.
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

	/// Returns an iterator over the connections input and output nodes.
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
