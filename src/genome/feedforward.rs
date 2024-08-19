use crate::{Config, genome::{Conn, Genome, Node}, Innov};
use rand::{seq::{IteratorRandom, SliceRandom}, Rng};
use std::{cell::{OnceCell, RefCell}, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, fmt, iter, rc::Rc};

/// The [feedforward](https://wikipedia.org/wiki/Feedforward_neural_network) implementation of the [`Genome`] trait.
#[derive(Clone)]
pub struct FeedForward {
	/// The set of connections in the genome.
	///
	/// We use a [`BTreeSet`] as the collection type for 3 main reasons:
	///
	/// 1. O(n)~ iteration with preserved ordering.
	/// 2. Relatively fast O(log(n)) retrievals, insertions, and removals.
	/// 3. Access to the [`BTreeSet::intersection`], [`BTreeSet::difference`], and [`BTreeSet::symmetric_difference`]
	///    functions that are useful during crossover.
	conns: BTreeSet<Rc<Conn>>,

	/// The genome's input nodes.
	input: Box<[Rc<Node>]>,

	/// The set of hidden nodes in the genome.
	///
	/// We use a [`HashSet`] as the collection type for 2 main reasons.
	///
	/// 1. O(n) iteration.
	/// 2. Extremely fast O(1)~ retrievals, insertions, and removals.
	hidden: HashSet<Rc<Node>>,

	/// The genome's output nodes.
	output: Box<[Rc<Node>]>,

	/// The genome's fitness.
	///
	/// We wrap the [`f32`] in a [`OnceCell`] so that we can ensure that the fitness is only initialized once, and so
	/// that we can initialize the fitness without a `&mut self`
	fitness: OnceCell<f32>,
}

impl FeedForward {
	/// Inserts a connection into the genome and returns an [`Rc`] of that connection.
	pub(crate) fn insert_conn(&mut self, conn: Conn) -> Rc<Conn> {
		let conn = Rc::new(conn);
		self.conns.insert(conn.clone());
		conn.clone()
	}

	/// Inserts a node into the genome and returns an [`Rc`] of that node.
	pub(crate) fn insert_node(&mut self, node: Node) -> Rc<Node> {
		let node = Rc::new(node);
		self.hidden.insert(node.clone());
		self.hidden.get(&node).cloned().unwrap()
	}

	/// Performs the add connection mutation using set parameters.
	pub(crate) fn add_conn(&mut self, input: Rc<Node>, output: Rc<Node>, weight: f32, innov: &Innov) -> Rc<Conn> {
		let new_conn = self.insert_conn(Conn::new(
			input.clone(),
			output.clone(),
			weight,
			innov.new_conn(input.clone(), output.clone())
		));

		input.insert_forward_conn(new_conn.clone());
		output.insert_backward_conn(new_conn.clone());

		new_conn
	}

	/// Performs the split connection mutation using set parameters.
	pub(crate) fn split_conn(&mut self, old_conn: Rc<Conn>, innov: &Innov) -> (Rc<Conn>, Rc<Conn>) {
		old_conn.disable();

		let new_node = self.insert_node(Node::new_hidden(innov.new_node()));

		let conn_a = self.insert_conn(Conn::new(
			old_conn.input(),
			new_node.clone(),
			1.0,
			innov.new_conn(old_conn.input(), new_node.clone())
		));

		let conn_b = self.insert_conn(Conn::new(
			new_node.clone(),
			old_conn.output(),
			old_conn.weight(),
			innov.new_conn(new_node.clone(), old_conn.output()),
		));

		old_conn.input().insert_forward_conn(conn_a.clone());
		new_node.insert_backward_conn(conn_a.clone());
		new_node.insert_forward_conn(conn_b.clone());
		old_conn.output().insert_backward_conn(conn_b.clone());

		(conn_a, conn_b)
	}

	/// Returns the genome's fitness.
	///
	/// # Panics
	///
	/// Panics if the fitness has not already been set.
	pub(crate) fn fitness(&self) -> f32 {
		let fitness = self.fitness.get();
		fitness.cloned().unwrap()
	}

	/// Returns an iterator over the genome's connections.
	pub(crate) fn iter_conns(&self) -> impl Iterator<Item = Rc<Conn>> {
		self.conns.iter().cloned().collect::<Vec<_>>().into_iter()
	}

	/// Returns an iterator over the genome's input nodes.
	#[allow(clippy::iter_cloned_collect)]
	pub(crate) fn iter_input(&self) -> impl Iterator<Item = Rc<Node>> {
		self.input.iter().cloned().collect::<Vec<_>>().into_iter()
	}

	/// Returns an iterator over the genome's hidden nodes.
	pub(crate) fn iter_hidden(&self) -> impl Iterator<Item = Rc<Node>> {
		self.hidden.iter().cloned().collect::<Vec<_>>().into_iter()
	}

	/// Returns an iterator over the genome's output nodes.
	#[allow(clippy::iter_cloned_collect)]
	pub(crate) fn iter_output(&self) -> impl Iterator<Item = Rc<Node>> {
		self.output.iter().cloned().collect::<Vec<_>>().into_iter()
	}
}

impl fmt::Debug for FeedForward {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = f.debug_struct("FeedForward");

		if !self.conns.is_empty() {
			output.field("conns", &self.iter_conns().collect::<Vec<_>>());
		}

		output.field("input", &self.iter_input().collect::<Vec<_>>());
		
		if !self.hidden.is_empty() {
			output.field("hidden", &self.iter_hidden().collect::<Vec<_>>());
		}

		output.field("output", &self.iter_output().collect::<Vec<_>>());
		
		if self.fitness.get().is_some() {
			output.field("fitness", &self.fitness());
		}

		output.finish()
	}
}

impl Genome for FeedForward {
	type Config = Config;
	type Innov = Innov;

	fn minimal(rng: &mut impl Rng, innov: &Self::Innov, config: &Self::Config) -> Self {
		let mut minimal = Self {
			conns: BTreeSet::new(),
			input: iter::repeat_with(|| Rc::new(Node::new_input(innov.new_node()))).take(config.num_inputs()).collect(),
			hidden: HashSet::new(),
			output: iter::repeat_with(|| Rc::new(Node::new_output(innov.new_node()))).take(config.num_outputs()).collect(),
			fitness: OnceCell::new(),
		};

		for input in minimal.iter_input() {
			for output in minimal.iter_output() {
				minimal.add_conn(input.clone(), output.clone(), rng.gen(), innov);
			}
		}

		minimal
	}

	fn mutate_add_conn(&mut self, rng: &mut impl Rng, innov: &Self::Innov, config: &Self::Config) {
		let mut inputs = self.iter_input().chain(self.iter_hidden()).collect::<Vec<_>>();
		inputs.shuffle(rng);

		let mut outputs = self.iter_hidden().chain(self.iter_output()).collect::<Vec<_>>();
		outputs.shuffle(rng);

		let input = inputs.into_iter().find(|node| {
			// (possible forward conns) - (node's forward conns) > 0 node has at least one valid output node.
			outputs.len()
				.saturating_sub(self.hidden.contains(node) as usize)
				.saturating_sub(node.num_forward_conns()) > 0
		}).unwrap();

		let output = outputs.into_iter().find(|node| !node.any_backward_conns(|conn| conn.input() == input)).unwrap();

		self.add_conn(input.clone(), output.clone(), rng.gen(), innov);
	}

	fn mutate_split_conn(&mut self, rng: &mut impl Rng, innov: &Self::Innov, config: &Self::Config) {
		assert_ne!(self.conns.len(), 0);
		let old_conn = self.iter_conns().filter(|conn| conn.enabled()).choose(rng).unwrap();
		self.split_conn(old_conn, innov);
	}

	fn mutate_conn_weight(&mut self, rng: &mut impl Rng, config: &Self::Config) {
		let weight_perturbation_preference = 9.0 / 10.0;

		let random_conn = self.iter_conns().filter(|conn| conn.enabled()).choose(rng).unwrap();
		match rng.gen_bool(weight_perturbation_preference) {
			true => random_conn.perturbe_weight(rng),
			false => random_conn.replace_weight(rng),
		}
	}

	fn activate(&self, inputs: impl AsRef<[f32]>, config: &Self::Config) -> impl AsRef<[f32]> {
		let inputs = inputs.as_ref();

		let mut nodes = BTreeMap::from_iter(self.iter_input().enumerate().map(|(i, node)| (node.clone(), inputs[i])));

		while let Some((node, val)) = nodes.pop_last() {
			// TODO: Apply activation function to `val`.

			for conn in node.iter_enabled_forward_conns() {
				*nodes.entry(conn.output()).or_default() += val * conn.weight();
			}

			if nodes.last_entry().unwrap().key().is_output() {
				break;
			}
		}

		(0..config.num_outputs()).map(|i| nodes.get(&self.output[i]).cloned().unwrap()).collect::<Vec<_>>()
	}

	fn set_fitness(&mut self, fitness: f32, config: &Self::Config) {
		self.fitness.set(fitness).unwrap();
	}

	fn compat_dist(lhs: &Self, rhs: &Self, config: &Self::Config) -> f32 {
		todo!();
	}

	fn crossover(lhs: Self, rhs: Self, rng: &mut impl Rng, config: &Self::Config) -> Self {
		// TODO: Update node rcs to point to new conns.
		// TODO: Make map from old to new conns to update easily.

		// TODO: Move these to a configuration structure.
		let matching_pref: f64 = 2.0 / 3.0;
		let disable_chance: f64 = 1.0 / 3.0;

		let rng = RefCell::new(rng);

		let ordered = {
			let mut pair = [&lhs, &rhs];
			pair.sort_by(|a, b| a.fitness().total_cmp(&b.fitness()));
			(pair[0], pair[1])
		};

		// Creates an iterator of the the parents' matching genes that will be used in the child's genome.
		let matching: Box<dyn Iterator<Item = Rc<Conn>>> = {
			if lhs.fitness() == rhs.fitness() {
				Box::new(lhs.conns.intersection(&rhs.conns).map(|key| {
					let random = rng.borrow_mut().gen();

					let parent = match random {
						false => &lhs,
						true => &rhs,
					};

					parent.conns.get(key).cloned().unwrap()
				}))
			} else {
				Box::new(lhs.conns.intersection(&rhs.conns).map(|key| {
					let random = rng.borrow_mut().gen_bool(matching_pref);

					let parent = match random {
						false => ordered.0,
						true => ordered.1,
					};

					parent.conns.get(key).cloned().unwrap()
				}))
			}
		};

		// Creates an iterator of the parents' disjoint genes that will be used in the child's genome.
		let disjoint: Box<dyn Iterator<Item = Rc<Conn>>> = {
			if lhs.fitness() == rhs.fitness() {
				Box::new(lhs.conns.symmetric_difference(&rhs.conns).filter_map(|conn| {
					let random = rng.borrow_mut().gen::<bool>();
					random.then_some(conn).cloned()
				}))
			} else {
				Box::new(ordered.1.conns.difference(&ordered.0.conns).cloned())
			}
		};

		// We need to clone the parents' conns because if the parents have multiple offspring, all of the offspring
		// will be sharing the same conns, which could lead to conflicting mutations.
		let new_conns: Vec<_> = matching.chain(disjoint).map(|conn| {
			Rc::new(Conn::clone(&conn))
		}).collect();

		let parents_nodes = HashSet::<_>::from_iter(new_conns.iter().flat_map(|conn| conn.nodes()));

		let parents_inputs = HashSet::<_>::from_iter(lhs.iter_input().chain(rhs.iter_input()));
		let input_intersection = parents_inputs.intersection(&parents_nodes);
		let input_map = HashMap::<_, _>::from_iter(input_intersection.map(|node| {
			(node.clone(), Rc::new(Node::clone(node)))
		}));

		let parents_outputs = HashSet::<_>::from_iter(lhs.iter_output().chain(rhs.iter_output()));
		let output_intersection = parents_outputs.intersection(&parents_nodes);
		let output_map = HashMap::<_, _>::from_iter(output_intersection.map(|node| {
			(node.clone(), Rc::new(Node::clone(node)))
		}));

		let parents_hidden = HashSet::<_>::from_iter(lhs.iter_hidden().chain(rhs.iter_hidden()));
		let hidden_intersection = parents_hidden.intersection(&parents_nodes);
		let hidden_map = HashMap::<_, _>::from_iter(hidden_intersection.map(|node| {
			(node.clone(), Rc::new(Node::clone(node)))
		}));

		let mut new_nodes = HashMap::<_, _>::new();
		new_nodes.extend(input_map.clone());
		new_nodes.extend(output_map.clone());
		new_nodes.extend(hidden_map.clone());

		for conn in new_conns.iter() {
			conn.set_input(|node| new_nodes.get(&node).cloned().unwrap().clone());
			conn.set_output(|node| new_nodes.get(&node).cloned().unwrap().clone());
		}

		Self {
			conns: BTreeSet::from_iter(new_conns),
			input: lhs.iter_input().zip(rhs.iter_input()).map(|(a, b)| {
				input_map.get(&a).or(input_map.get(&b)).cloned().unwrap()
			}).collect(),
			hidden: HashSet::from_iter(hidden_map.values().cloned()),
			output: lhs.iter_output().zip(rhs.iter_output()).map(|(a, b)| {
				output_map.get(&a).or(output_map.get(&b)).cloned().unwrap()
			}).collect(),
			fitness: OnceCell::new(),
		}
	}
}
