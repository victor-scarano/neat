extern crate alloc;
use crate::{genome::Genome, node::*, pop::Pop};
use core::{cell::Cell, cmp::Ordering, fmt, hash, iter, mem};
use alloc::{collections::BTreeSet, rc::*};
use hashbrown::HashSet;
use rand::{seq::IteratorRandom, Rng};

#[derive(Clone)]
pub struct Edge {
    pub tail: Tail,
    pub head: Head,
    pub weight: f32,
    pub enabled: Cell<bool>,
    pub layer: usize,
    pub innov: usize,
}

impl Edge {
    pub fn new(tail: impl Into<Tail>, head: impl Into<Head>) -> Self {
        let tail = tail.into();
        let head = head.into();

        assert_ne!(tail, head);

        head.update_layer(tail.layer() + 1);

        Self {
            innov: Pop::next_edge_innov(&tail, &head),
            layer: tail.layer(),
            enabled: Cell::new(true),
            weight: 1.0,
            tail,
            head,
        }
    }

    pub fn split(&self) -> (Edge, Edge) {
        let middle = Hidden::from_edge(self);
        let first = Edge::new(self.tail.clone(), middle.clone());
        let last = Edge::new(middle, self.head.clone());
        (first, last)
    }
}

impl Eq for Edge {}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f
            .debug_struct("Edge")
            .field_with("tail", |f| fmt::Pointer::fmt(&self.tail, f))
            .field_with("head", |f| fmt::Pointer::fmt(&self.head, f))
            .field("weight", &self.weight)
            .field("enabled", &self.enabled.get())
            .field("layer", &self.layer)
            .field("innov", &self.innov)
            .finish()
    }
}

impl hash::Hash for Edge {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.innov.hash(state);
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.enabled.get()
        //    .cmp(&other.enabled.get())
        //    .reverse()
        self.layer.cmp(&other.layer).then(self.innov.cmp(&other.innov))
    }
}

// used to be equal if innovations were equal, but needs to reflect ord impl
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq() && self.innov == other.innov
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// TODO: Write custom Rc implementation to optimize for only two possible references so that the RcInner allocation
// isn't as large as it normally is
pub struct Edges {
    btree_set: BTreeSet<Rc<Edge>>,
    hash_set: HashSet<Rc<Edge>>,
}

impl Edges {
    pub fn new() -> Self {
        Self {
            btree_set: BTreeSet::new(),
            hash_set: HashSet::new(),
        }
    }

    pub fn get(&self, edge: &Edge) -> &Edge {
        self.hash_set.get(edge).unwrap()
    }

    pub fn insert(&mut self, edge: Edge) {
        let edge = Rc::new(edge);

        let inserted = self.btree_set.insert(edge.clone());
        assert!(inserted);

        let inserted = self.hash_set.insert(edge.clone());
        assert!(inserted);
    }

    pub fn random_edges(&self, rng: &mut impl Rng) -> (&Edge, &Edge) {
        // returns two random nonequal edges
        assert!(self.len() >= 1);

        let mut edges = loop {
            let edges = self.iter_ordered().choose_multiple(rng, 2);

            if edges[0] != edges[1] {
                break edges;
            }
        };

        edges.sort_unstable();

        (edges[0], edges[1])
    }

    pub fn iter_ordered(&self) -> impl Iterator<Item = &Edge> {
        self.btree_set.iter().map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn iter_unordered(&self) -> impl Iterator<Item = &Edge> {
        self.hash_set.iter().map(<Rc<Edge> as AsRef<Edge>>::as_ref)
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.btree_set.len(), self.hash_set.len());
        self.hash_set.len() // need to check if one is faster than the other
    }

    pub fn crossover<const I: usize, const O: usize>(mut lhs: Genome<I, O>, mut rhs: Genome<I, O>, rng: &mut impl Rng) -> Genome<I, O> {
        const TEMP_MATCHING_PREF: f64 = 2.0 / 3.0;

        // order parents based on fitness
        if lhs.fitness > rhs.fitness {
            mem::swap(&mut lhs, &mut rhs);
        }

        let bump = Bump::new();

        let mut inputs = HashSet::new();
        let mut hiddens = HashSet::new();
        let mut outputs = HashSet::new();

        let mut edges = Edges::new();

        lhs.edges.hash_set.intersection(&rhs.edges.hash_set).map(|key| {
            let choice = match lhs.fitness == rhs.fitness {
                false => rng.gen_bool(TEMP_MATCHING_PREF),
                true => rng.gen(),
            };

            let parent = match choice { false => &lhs, true => &rhs };

            let edge = parent.edges.get(key);

            let tail = match edge.tail {
                Tail::Input(ref input) => Tail::Input(inputs.get_or_insert(input.clone_in(bump.clone())).clone()),
                Tail::Hidden(ref hidden) => Tail::Hidden(hiddens.get_or_insert(hidden.clone_in(bump.clone())).clone()),
            };

            let head = match edge.head {
                Head::Hidden(ref hidden) => Head::Hidden(hiddens.get_or_insert(hidden.clone_in(bump.clone())).clone()),
                Head::Output(ref output) => Head::Output(outputs.get_or_insert(output.clone_in(bump.clone())).clone()),
            };

            Edge::new(tail, head)
        }).collect_into(&mut edges);

        match lhs.fitness == rhs.fitness {
            false => rhs.edges.hash_set.difference(&rhs.edges.hash_set).map(|edge| {
                let tail = match edge.tail {
                    Tail::Input(ref input) => Tail::Input(inputs.get_or_insert(input.clone_in(bump.clone())).clone()),
                    Tail::Hidden(ref hidden) => Tail::Hidden(hiddens.get_or_insert(hidden.clone_in(bump.clone())).clone()),
                };

                let head = match edge.head {
                    Head::Hidden(ref hidden) => Head::Hidden(hiddens.get_or_insert(hidden.clone_in(bump.clone())).clone()),
                    Head::Output(ref output) => Head::Output(outputs.get_or_insert(output.clone_in(bump.clone())).clone()),
                };

                Edge::new(tail, head)
            }).collect_into(&mut edges),

            true => lhs.edges.hash_set.symmetric_difference(&rhs.edges.hash_set).filter(|_| rng.gen()).map(|edge| {
                let tail = match edge.tail {
                    Tail::Input(ref input) => Tail::Input(inputs.get_or_insert(input.clone_in(bump.clone())).clone()),
                    Tail::Hidden(ref hidden) => Tail::Hidden(hiddens.get_or_insert(hidden.clone_in(bump.clone())).clone()),
                };

                let head = match edge.head {
                    Head::Hidden(ref hidden) => Head::Hidden(hiddens.get_or_insert(hidden.clone_in(bump.clone())).clone()),
                    Head::Output(ref output) => Head::Output(outputs.get_or_insert(output.clone_in(bump.clone())).clone()),
                };

                Edge::new(tail, head)
            }).collect_into(&mut edges),
        };

        let mut inputs = inputs.into_iter().collect::<Vec<_>>();
        inputs.sort_unstable_by_key(|input| input.index());
        let inputs = Inputs::try_from(inputs).unwrap();

        let mut outputs = outputs.into_iter().collect::<Vec<_>>();
        outputs.sort_unstable_by_key(|output| output.index::<I>());
        let outputs = Outputs::try_from(outputs).unwrap();

        Genome { bump, inputs, outputs, edges, fitness: 0.0 }
    }
}

impl iter::Extend<Edge> for Edges {
    fn extend<T: IntoIterator<Item = Edge>>(&mut self, iter: T) {
        let mut iter = iter.into_iter().map(Rc::new);
        self.btree_set.extend(&mut iter);
        self.hash_set.extend(&mut iter);
    }
}

impl fmt::Debug for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter_ordered()).finish()
    }
}
