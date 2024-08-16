
use std::{cell::{Cell, RefCell}, cmp::Ordering, collections::BTreeSet, fmt, hash::{Hash, Hasher}, rc::Rc};

/// A connection between two [`Node`]s (also known as neurons) within the genome.
#[derive(Default)]
pub(crate) struct Conn {
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
	weight: f32,

	/// The enabled status of the connection.
	///
	/// We wrap the [`bool`] in a [`Cell`] to provide interior mutability of the enabled status, since most method
	/// calls to `Self` are handled through an [`Rc`].
	enabled: Cell<bool>,

	/// The innovation of the connection.
	innov: u32,
}

impl Conn {
	/// Constructs a new connection.
	///
	/// The input and output parameters to this function are [`Rc<Node>`]s instead of [`Node`]s because it (somewhat)
	/// ensures that they have been already been inserted into the genome.
	pub fn new(input: Rc<Node>, output: Rc<Node>, weight: f32, innov: u32) -> Self {
		Self {
			input: Cell::new(input),
			output: Cell::new(output),
			weight,
			enabled: Cell::new(true),
			innov,
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
		self.weight
	}

	/// Returns the enabled status of the connection.
	pub fn enabled(&self) -> bool {
		self.enabled.get()
	}

	/// Disables the connection.
	pub fn disable(&self) {
		self.enabled.set(false);
	}

	/// Returns an iterator over the connections input and output nodes.
	pub fn nodes(&self) -> impl Iterator<Item = Rc<Node>> {
		[self.input(), self.output()].into_iter()
	}
}

impl Clone for Conn {
	fn clone(&self) -> Self {
		Self {
			input: Cell::new(self.input()),
			output: Cell::new(self.output()),
			weight: self.weight,
			enabled: self.enabled.clone(),
			innov: self.innov,
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
			.field("innov", &self.innov)
			.finish()
	}
}

impl Hash for Conn {
	fn hash<H: Hasher>(&self, state: &mut H) {
		Rc::as_ptr(&self.input()).hash(state);
		Rc::as_ptr(&self.output()).hash(state);
	}
}

impl Ord for Conn {
	fn cmp(&self, other: &Self) -> Ordering {
		self.innov.cmp(&other.innov)
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

/// A node or neuron within the genome, containing a set of both the forwards and backwards facing connections.
#[derive(Default, Clone, Eq)]
pub(crate) struct Node {
	/// Specifies the position of the node.
	kind: NodeKind,

	/// The set of [`Conn`]s such that this [`Node`] is the input of the connection.
	///
	/// We wrap [`BTreeSet<Rc<Conn>>`] in a [`RefCell`] to provide interior mutability to the set.
	forward: RefCell<BTreeSet<Rc<Conn>>>,

	/// The set of [`Conn`]s such that this [`Node`] is the output of the connection.
	///
	/// We wrap [`BTreeSet<Rc<Conn>>`] in a [`RefCell`] to provide interior mutability to the set.
	backward: RefCell<BTreeSet<Rc<Conn>>>,
}

impl Node {
	/// Constructs a new node representing an input node within a genome.
	pub(crate) fn new_input() -> Self {
		Self {
			kind: NodeKind::Input,
			forward: RefCell::new(BTreeSet::new()),
			backward: RefCell::new(BTreeSet::new()),
		}
	}

	/// Constructs a new node representing a hidden node within a genome.
	pub(crate) fn new_hidden() -> Self {
		Self {
			kind: NodeKind::Hidden,
			forward: RefCell::new(BTreeSet::new()),
			backward: RefCell::new(BTreeSet::new()),
		}
	}

	/// Constructs a new node representing an output node within a genome.
	pub(crate) fn new_output() -> Self {
		Self {
			kind: NodeKind::Output,
			forward: RefCell::new(BTreeSet::new()),
			backward: RefCell::new(BTreeSet::new()),
		}
	}

	/// Returns true if the node represents an input node.
	pub(crate) fn is_input(&self) -> bool {
		self.kind == NodeKind::Input
	}

	/// Returns true if the node represents a hidden node.
	pub(crate) fn is_hidden(&self) -> bool {
		self.kind == NodeKind::Hidden
	}

	/// Returns true if the node represents an output node.
	pub(crate) fn is_output(&self) -> bool {
		self.kind == NodeKind::Output
	}

	/// Inserts a forwards facing connection if the node does not represent an output node.
	pub(crate) fn insert_forward_conn(&self, conn: Rc<Conn>) {
		if self.kind != NodeKind::Output {
			self.forward.borrow_mut().insert(conn);
		}
	}

	/// Inserts a backwards facing connection if the node does not represent an input node.
	pub(crate) fn insert_backward_conn(&self, conn: Rc<Conn>) {
		if self.kind != NodeKind::Input {
			self.backward.borrow_mut().insert(conn);
		}
	}

	/// Returns the number of forward facing connections.
	pub(crate) fn num_forward_conns(&self) -> usize {
		self.forward.borrow().len()
	}

	/// Returns the number of backward facing connections.
	pub(crate) fn num_backward_conns(&self) -> usize {
		self.backward.borrow().len()
	}

	/// Iterates over the node's forward facing connections that are enabled.
	pub(crate) fn iter_enabled_forward_conns(&self) -> impl Iterator<Item = Rc<Conn>> + '_ {
		self.forward.borrow().iter().filter(|conn| conn.enabled()).cloned().collect::<Vec<_>>().into_iter()
	}

	/// Returns true if a connection exists matching the predicate in the node's backward facing connections.
	pub(crate) fn any_backward_conns(&self, f: impl FnMut(&Rc<Conn>) -> bool) -> bool {
		self.backward.borrow().iter().any(f)
	}
}

impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = f.debug_struct("Node");

		output.field("addr", &format_args!("{:?}", self as *const Self));
		output.field("kind", &self.kind);

		if self.kind != NodeKind::Output {
			output.field("forward", &self.forward.borrow().iter().map(|conn| format!("{:p}", *conn)).collect::<Vec<_>>());
		}

		if self.kind != NodeKind::Input {
			output.field("backward", &self.backward.borrow().iter().map(|conn| format!("{:p}", *conn)).collect::<Vec<_>>());
		}

		output.finish()
	}
}

impl Hash for Node {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.kind.hash(state);
		self.forward.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
		self.backward.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.kind == other.kind {
			self.num_backward_conns().cmp(&other.num_backward_conns()).reverse()
		} else {
			self.kind.cmp(&other.kind).reverse()
		}
	}
}

impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.kind == other.kind &&
		self.forward.borrow().iter().zip(other.forward.borrow().iter()).all(|(a, b)| Rc::ptr_eq(a, b)) &&
		self.backward.borrow().iter().zip(other.backward.borrow().iter()).all(|(a, b)| Rc::ptr_eq(a, b))
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

/// Specifies the position of a [`Node`] in a genome.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum NodeKind {
	/// Represents an input node of a genome.
	Input,

	/// Represents a hidden node of a genome.
	#[default]
	Hidden,

	/// Represents an output node of a genome.
	Output
}
