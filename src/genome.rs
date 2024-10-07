use crate::{conn::Conn, node::*};
use std::{array, collections::*, fmt};
use rand::{Rng, seq::IteratorRandom};

pub struct Genome<const I: usize, const O: usize> {
    input: Box<[Input; I]>,
    hidden: HashSet<Hidden>, // may need to wrap hidden in a pinned box to prevent ub in case of hashset reallocations
    output: Box<[Output; O]>,
    pub conns: BTreeSet<Conn>,
    fitness: f32,
}

impl<const I: usize, const O: usize> Genome<I, O> {
    pub fn new() -> Self {
        Self {
            input: Box::new(array::from_fn::<_, I, _>(|_| Input::new())),
            hidden: HashSet::new(),
            output: Box::new(array::from_fn::<_, O, _>(|_| Output::new())),
            conns: BTreeSet::new(),
            fitness: f32::default(),
        }
    }

    pub fn mutate_add_conn(&mut self, rng: &mut impl Rng) {
        let leading = self.input.iter().map(Leading::from)
            .chain(self.hidden.iter().map(Leading::from))
            .choose(rng).unwrap();

        let trailing = self.hidden.iter().map(Trailing::from)
            .chain(self.output.iter().map(Trailing::from))
            .filter(|trailing| *trailing != leading)
            .choose(rng).unwrap();

        self.conns.insert(Conn::new(leading, trailing));
    }

    pub fn mutate_split_conn(&mut self, rng: &mut impl Rng) {
        let conn = self.conns.iter().filter(|conn| conn.enabled()).choose(rng).unwrap();
        conn.disable();

        let middle = self.hidden.get_or_insert(Hidden::new(conn));

        let first = Conn::new(conn.leading(), Trailing::from(middle));
        let last = Conn::new(Leading::from(middle), conn.trailing());

        self.conns.insert(first);
        self.conns.insert(last);
    }

    pub fn mutate_weight(&mut self) {
        todo!();
    }

    pub fn activate(&self, inputs: impl AsRef<[f32; I]>) -> [f32; O] {
        // activation(bias + (response * aggregation(inputs)))
        // input nodes have: activation=identity, response=1, agreggation=none
        todo!();
    }

    pub fn compat_dist(&self) -> f32 {
        todo!();
    }

    pub fn crossover(lhs: Self, rhs: Self) -> Self {
        todo!();
    }
}

impl<const I: usize, const G: usize> fmt::Debug for Genome<I, G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Genome")
            .field_with("Connections", |f| {
                f.debug_list()
                    .entries(self.conns.iter())
                    .finish()
            })
            .field_with("Input Nodes", |f| {
                f.debug_map()   
                    .entries(self.input.iter().map(|input| (input as *const _, input)))
                    .finish()
            })
            .field_with("Hidden Nodes", |f| {
                f.debug_map()   
                    .entries(self.hidden.iter().map(|hidden| (hidden as *const _, hidden)))
                    .finish()
            })
            .field_with("Output Nodes", |f| {
                f.debug_map()
                    .entries(self.output.iter().map(|output| (output as *const _, output)))
                    .finish()
            })
            .field("Fitness", &self.fitness)
            .finish()
    }
}

