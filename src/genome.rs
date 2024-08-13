use crate::{conn::Conn, node::Node};
use rand::{seq::{IteratorRandom, SliceRandom}, Rng};
use std::{
    array, cell::{OnceCell, RefCell}, collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt, iter, num::Saturating, ops::AddAssign, rc::Rc,
};

pub(crate) trait Genome<const I: usize, const O: usize> {
    /// Constructs a 'minimal' genome with no hidden nodes.
    fn minimal() -> Self;

    /// A single new connection with a random weight is added connecting two previously unconnected nodes.
    fn mutate_add_conn(&mut self, rng: &mut impl Rng);

    /// An existing connection is split and the new node placed where the old connection used to be. The old connection
    /// is disabled and two new connections are added to the genome. The new connection leading into the new node
    /// receives a weight of 1.0, and the new connection leading out receives the same weight as the old connection.
    ///
    /// # Note
    ///
    /// In Stanley's NEAT paper, this is referred to as the 'add node' mutation. I refer to it as the 'add connection'
    /// mutation in my implementation because I believe it is easier to understand that way.
    fn mutate_split_conn(&mut self, rng: &mut impl Rng);

    fn mutate_conn_weight(&mut self, rng: &mut impl Rng);

    fn activate(&self, inputs: [f32; I]) -> [f32; O];

    /// Sets the genome's fitness.
    fn set_fitness(&mut self, fitness: f32);

    /// Computes the compatibility distance between two genomes.
    fn compat_dist(lhs: &Self, rhs: &Self) -> f32;

    /// Consumes two parent genomes and returns a child genome.
    fn crossover(lhs: Self, rhs: Self, rng: &mut impl Rng) -> Self;
}

#[derive(Clone)]
pub struct FeedForward<const I: usize, const O: usize> {
    conns: BTreeSet<Rc<Conn>>,
    input: Box<[Rc<Node>]>,
    hidden: BTreeSet<Rc<Node>>,
    output: Box<[Rc<Node>]>,
    fitness: OnceCell<f32>,
}

impl<const I: usize, const O: usize> FeedForward<I, O> {
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
    pub(crate) fn add_conn(&mut self, input: Rc<Node>, output: Rc<Node>, weight: f32, innov: u32) -> Rc<Conn> {
        let new_conn = self.insert_conn(Conn::new(input.clone(), output.clone(), weight, innov));
        input.insert_forward_conn(new_conn.clone());
        output.insert_backward_conn(new_conn.clone());
        new_conn.clone()
    }

    /// Performs the split connection mutation using set parameters.
    pub(crate) fn split_conn(&mut self, old_conn: Rc<Conn>, innov_a: u32, innov_b: u32) {
        old_conn.disable();

        let new_node = self.insert_node(Node::new_hidden());

        let conn_a = self.insert_conn(Conn::new(old_conn.input(), new_node.clone(), 1.0, innov_a));
        let conn_b = self.insert_conn(Conn::new(
            new_node.clone(),
            old_conn.output(),
            old_conn.weight(),
            innov_b,
        ));

        new_node.insert_forward_conn(conn_b.clone());
        new_node.insert_backward_conn(conn_a.clone());

        if old_conn.input().is_input() {
            old_conn.input().insert_forward_conn(conn_a.clone());
        }

        if old_conn.output().is_output() {
            old_conn.output().insert_backward_conn(conn_b.clone());
        }
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
    pub(crate) fn iter_conns(&self) -> impl Iterator<Item = Rc<Conn>> + '_ {
        self.conns.iter().cloned()
    }

    /// Returns an iterator over the genome's input nodes.
    pub(crate) fn iter_input(&self) -> impl Iterator<Item = Rc<Node>> + '_ {
        self.input.iter().cloned()
    }

    /// Returns an iterator over the genome's hidden nodes.
    pub(crate) fn iter_hidden(&self) -> impl Iterator<Item = Rc<Node>> + '_ {
        self.hidden.iter().cloned()
    }

    /// Returns an iterator over the genome's output nodes.
    pub(crate) fn iter_output(&self) -> impl Iterator<Item = Rc<Node>> + '_ {
        self.output.iter().cloned()
    }
}

impl<const I: usize, const O: usize> fmt::Debug for FeedForward<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = f.debug_struct("FeedForward");

        if self.conns.len() > 0 {
            output.field("conns", &self.iter_conns().map(|conn| format!("{:?}", conn)).collect::<Vec<_>>());
        }

        output.field("input", &self.iter_input().map(|node| format!("{:?}", node)).collect::<Vec<_>>());
        
        if self.hidden.len() > 0 {
            output.field("hidden", &self.iter_hidden().map(|node| format!("{:?}", node)).collect::<Vec<_>>());
        }

        output.field("output", &self.iter_output().map(|node| format!("{:?}", node)).collect::<Vec<_>>());
        
        if self.fitness.get().is_some() {
            output.field("fitness", &self.fitness());
        }

        output.finish()
    }
}

impl<const I: usize, const O: usize> Genome<I, O> for FeedForward<I, O> {
    fn minimal() -> Self {
        debug_assert_ne!(I, 0);
        debug_assert_ne!(O, 0);

        Self {
            conns: BTreeSet::new(),
            input: iter::repeat_with(|| Rc::new(Node::new_input())).take(I).collect(),
            hidden: BTreeSet::new(),
            output: iter::repeat_with(|| Rc::new(Node::new_output())).take(O).collect(),
            fitness: OnceCell::new(),
        }
    }

    fn mutate_add_conn(&mut self, rng: &mut impl Rng) {
        let mut inputs = self.iter_input().chain(self.iter_hidden()).collect::<Vec<_>>();
        inputs.shuffle(rng);

        let mut outputs = self.iter_hidden().chain(self.iter_output()).collect::<Vec<_>>();
        outputs.shuffle(rng);

        let input = inputs.into_iter().find(|node| {
            let mut count = Saturating(outputs.len());
            count -= self.hidden.contains(node) as usize;
            count -= node.num_forward_conns();
            count.0 > 0
        }).unwrap();

        let output = outputs.into_iter().find(|node| !node.any_backward_conns(|conn| conn.input() == input)).unwrap();

        self.add_conn(input, output, rng.gen(), u32::MAX);
    }

    fn mutate_split_conn(&mut self, rng: &mut impl Rng) {
        debug_assert_ne!(self.conns.len(), 0);
        let old_conn = self.iter_conns().filter(|conn| conn.enabled()).choose(rng).unwrap();
        self.split_conn(old_conn, u32::MAX, u32::MAX);
    }

    fn mutate_conn_weight(&mut self, rng: &mut impl Rng) {
        todo!();
    }

    fn activate(&self, inputs: [f32; I]) -> [f32; O] {
        let mut nodes = BTreeMap::from_iter(
            self.iter_input().enumerate().map(|(i, node)| (node.clone(), inputs[i])),
        );

        while let Some((node, val)) = nodes.pop_last() {
            // TODO: Apply activation function to `val`

            for conn in node.iter_enabled_forward_conns() {
                nodes.entry(conn.output()).or_default().add_assign(val * conn.weight());
            }

            if nodes.last_entry().unwrap().key().is_output() {
                break;
            }
        }

        array::from_fn::<_, O, _>(|i| nodes.get(&self.output[i]).cloned().unwrap())
    }

    fn set_fitness(&mut self, fitness: f32) {
        self.fitness.set(fitness).unwrap();
    }

    fn compat_dist(_lhs: &Self, _rhs: &Self) -> f32 {
        todo!();
    }

    fn crossover(lhs: Self, rhs: Self, rng: &mut impl Rng) -> Self {
        // TODO: UPDATE NODE RCS TO POINT TO NEW CONNS!!!
        // MAKE MAP FROM OLD TO NEW CONNS TO UPDATE EASILY!!!

        const MATCHING_PREF: f64 = 2.0 / 3.0;
        const DISABLE_CHANCE: f64 = 1.0 / 3.0;

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
                    let random = rng.borrow_mut().gen_bool(MATCHING_PREF);

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
            hidden: BTreeSet::from_iter(hidden_map.values().cloned()),
            output: lhs.iter_output().zip(rhs.iter_output()).map(|(a, b)| {
                output_map.get(&a).or(output_map.get(&b)).cloned().unwrap()
            }).collect(),
            fitness: OnceCell::new(),
        }
    }
}
