extern crate alloc;

use crate::{conn::Conn, node::*};
use core::{array, fmt};
use alloc::{boxed::Box, collections::BTreeSet, rc::Rc, vec::Vec};
use hashbrown::{HashMap, HashSet};
use rand::{Rng, seq::IteratorRandom};

pub struct Genome<const I: usize, const O: usize> {
    input: Box<[Rc<Input>; I]>,
    hidden: HashSet<Rc<Hidden>>,
    output: Box<[Rc<Output>; O]>,
    conns: BTreeSet<Conn>,
    fitness: f32,
}

impl<const I: usize, const O: usize> Genome<I, O> {
    pub fn new() -> Self {
        assert_ne!(I, 0);
        assert_ne!(O, 0);

        Self {
            input: Box::new(array::from_fn::<_, I, _>(|idx| Input::new(idx))),
            hidden: HashSet::new(),
            output: Box::new(array::from_fn::<_, O, _>(|_| Output::new())),
            conns: BTreeSet::new(),
            fitness: f32::default(),
        }
    }

    // currently tries to create the same connection if the rng does not change
    // between calls.
    // either we error if this happens, forcing the caller to call the function
    // again with a different rng.
    // OR we default to finding the first available connection, and if we cant
    // find any, then we error.
    pub fn mutate_add_conn(&mut self, rng: &mut impl Rng) {
        let leading = self.input.iter().map(Leading::from)
            .chain(self.hidden.iter().map(Leading::from))
            .choose_stable(rng)
            .unwrap();

        let trailing = self.hidden.iter().map(Trailing::from)
            .chain(self.output.iter().map(Trailing::from))
            .filter(|trailing| *trailing != leading)
            .choose_stable(rng)
            .unwrap();

        let conn = Conn::new(leading, trailing);
        assert!(self.conns.insert(conn));
    }

    pub fn mutate_split_conn(&mut self, rng: &mut impl Rng) {
        let conn = self.conns.iter()
            .filter(|conn| conn.enabled.get())
            .choose(rng)
            .unwrap();

        conn.enabled.set(false);

        let middle = self.hidden.get_or_insert(Hidden::new(conn));

        let first = Conn::new(&conn.leading, middle);
        let last = Conn::new(middle, &conn.trailing);

        self.conns.insert(first);
        self.conns.insert(last);
    }

    pub fn mutate_weight(&mut self) {
        todo!();
    }

    // weight * activation(bias + (response * aggregation(inputs)))
    // input nodes have: activation=identity, response=1, agreggation=none
    // is there a way we can cut down on space complexity here?
    // compare current time/space complexity with old implementation.
    // i think after a small space complexity optimization, this implementation
    // will be the best of both worlds.
    pub fn activate(&self, inputs: [f32; I]) -> [f32; O] {
        let mut map = HashMap::<Trailing, Accumulator>::new();

        for layer in self.conns.iter().filter(|conn| conn.enabled.get()) {
            match layer.leading {
                Leading::Input(ref input) => {
                    map
                        .entry(layer.trailing.clone())
                        .or_default()
                        .add(layer.weight * (input.bias() + inputs[input.idx]));
                }
                Leading::Hidden(ref hidden) => {
                    let aggregation = map
                        .entry(Trailing::from(hidden))
                        .or_default()
                        .get_or_aggregate(hidden.aggregator);

                    map
                        .entry(layer.trailing.clone())
                        .or_default()
                        .add(layer.weight * (hidden.bias() + (hidden.response() * aggregation)));
                }
            }
        }

        array::from_fn::<_, O, _>(|idx| {
            let output = &self.output[idx];

            let aggregation = map
                .entry(Trailing::from(output))
                .or_default()
                .get_or_aggregate(output.aggregator);

            output.bias() + (output.response() * aggregation)
        })
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
            .field_with("Connections", |f| f.debug_list().entries(self.conns.iter()).finish())
            .field_with("Input Nodes", |f| self.input.iter().fold(&mut f.debug_map(), |f, input| {
                f.key_with(|f| fmt::Pointer::fmt(input, f)).value(input)
            }).finish())
            .field_with("Hidden Nodes", |f| self.hidden.iter().fold(&mut f.debug_map(), |f, hidden| {
                f.key_with(|f| fmt::Pointer::fmt(hidden, f)).value(hidden)
            }).finish())
            .field_with("Output Nodes", |f| self.output.iter().fold(&mut f.debug_map(), |f, output| {
                f.key_with(|f| fmt::Pointer::fmt(output, f)).value(output)
            }).finish())
            .field("Fitness", &self.fitness)
            .finish()
    }
}

enum Accumulator {
    Accumulating(Vec<f32>),
    Aggregated(f32),
}

impl Accumulator {
    fn add(&mut self, value: f32) {
        if let Self::Accumulating(accum) = self {
            accum.push(value);
        }
    }

    fn get_or_aggregate(&mut self, aggregator: fn(&[f32]) -> f32) -> f32 {
        match self {
            Self::Accumulating(accum) => {
                let aggregated = aggregator(accum.as_slice());
                *self = Self::Aggregated(aggregated);
                aggregated
            }
            Self::Aggregated(eval) => *eval
        }
    }
}

impl Default for Accumulator {
    fn default() -> Self {
        Self::Accumulating(Vec::new())
    }
}
