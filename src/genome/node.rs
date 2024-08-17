use super::Conn;
use std::{cell::RefCell, cmp::Ordering, collections::BTreeSet, fmt, hash, rc::Rc};

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

	/// The innovation of the node.
	innov: u32,
}

impl Node {
	/// Constructs a new node representing an input node within a genome.
	pub(crate) fn new_input(innov: u32) -> Self {
		Self {
			kind: NodeKind::Input,
			forward: RefCell::new(BTreeSet::new()),
			backward: RefCell::new(BTreeSet::new()),
			innov,
		}
	}

	/// Constructs a new node representing a hidden node within a genome.
	pub(crate) fn new_hidden(innov: u32) -> Self {
		Self {
			kind: NodeKind::Hidden,
			forward: RefCell::new(BTreeSet::new()),
			backward: RefCell::new(BTreeSet::new()),
			innov,
		}
	}

	/// Constructs a new node representing an output node within a genome.
	pub(crate) fn new_output(innov: u32) -> Self {
		Self {
			kind: NodeKind::Output,
			forward: RefCell::new(BTreeSet::new()),
			backward: RefCell::new(BTreeSet::new()),
			innov,
		}
	}

	pub(crate) fn innov(&self) -> u32 {
		self.innov
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

		output.field("innov", &self.innov);

		output.finish()
	}
}

impl hash::Hash for Node {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
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
