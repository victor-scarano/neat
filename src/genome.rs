extern crate alloc;

use crate::{conn::Conn, node::*};
use core::{array, fmt};
use alloc::{boxed::Box, collections::{BTreeMap, BTreeSet}, rc::Rc};
use hashbrown::HashSet;
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
        assert!(I > 0);
        assert!(O > 0);

        Self {
            input: Box::new(array::from_fn::<_, I, _>(|idx| Input::new(idx))),
            hidden: HashSet::new(),
            output: Box::new(array::from_fn::<_, O, _>(|_| Output::new())),
            conns: BTreeSet::new(),
            fitness: f32::default(),
        }
    }

    pub fn mutate_add_conn(&mut self, rng: &mut impl Rng) {
        let leading = self.input.iter().map(Leading::from)
            .chain(self.hidden.iter().map(Leading::from))
            .choose(rng)
            .expect("self.input should be non-zero");

        let trailing = self.hidden.iter().map(Trailing::from)
            .chain(self.output.iter().map(Trailing::from))
            .filter(|trailing| *trailing != leading)
            .choose(rng)
            .expect("self.output should be non-zero");

        self.conns.insert(Conn::new(leading, trailing));
    }

    pub fn mutate_split_conn(&mut self, rng: &mut impl Rng) {
        assert!(self.conns.len() > 0);

        let conn = self.conns.iter()
            .filter(|conn| conn.enabled())
            .choose(rng)
            .expect("self.conns should contain at least one enabled conn");
        conn.disable();

        // this should always insert
        let middle = self.hidden.get_or_insert(Hidden::new(conn));

        let first = Conn::new(conn.leading(), Trailing::from(middle));
        let last = Conn::new(Leading::from(middle), conn.trailing());

        let inserted = self.conns.insert(first);
        assert!(inserted);
        let inserted = self.conns.insert(last);
        assert!(inserted);
    }

    pub fn mutate_weight(&mut self) {
        todo!();
    }

    pub fn activate(&self, inputs: impl AsRef<[f32; I]>) -> [f32; O] {
        // activation(bias + (response * aggregation(inputs)))
        // input nodes have: activation=identity, response=1, agreggation=none
        
        let inputs = inputs.as_ref();

        let mut map = BTreeMap::<_, f32>::new();

        for (conn, val) in self.conns.iter().filter_map(|conn| {
            conn.enabled()
                .then(|| conn.leading().input())
                .flatten()
                .map(|input| (conn, inputs[input.idx]))
        }) {
            *map.entry(conn.trailing()).or_default() += conn.leading().bias() + val;
        }

        for conn in self.conns.iter().filter(|conn| conn.level != 0) {
            todo!();
        }

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
            .field_with("Connections", |f| f.debug_list().entries(self.conns.iter()).finish())
            .field_with("Input Nodes", |f| self.input.iter().fold(&mut f.debug_map(), |f, input| {
                f.key_with(|f| fmt::Pointer::fmt(&input, f)).value(input)
            }).finish())
            .field_with("Hidden Nodes", |f| self.hidden.iter().fold(&mut f.debug_map(), |f, hidden| {
                f.key_with(|f| fmt::Pointer::fmt(&hidden, f)).value(hidden)
            }).finish())
            .field_with("Output Nodes", |f| self.output.iter().fold(&mut f.debug_map(), |f, output| {
                f.key_with(|f| fmt::Pointer::fmt(&output, f)).value(output)
            }).finish())
            .field("Fitness", &self.fitness)
            .finish()
    }
}

