use crate::{conn::Conn, node::*};
use std::{array, collections::*, fmt, pin::Pin};
use rand::{Rng, seq::IteratorRandom};

pub struct Genome<const I: usize, const O: usize> {
    input: Pin<Box<[Input; I]>>,
    hidden: HashSet<Pin<Box<Hidden>>>,
    output: Pin<Box<[Output; O]>>,
    conns: BTreeSet<Conn>,
    fitness: f32,
}

impl<const I: usize, const O: usize> Genome<I, O> {
    pub fn new() -> Self {
        Self {
            input: Box::into_pin(Box::new(array::from_fn::<_, I, _>(|_| Input::new()))),
            hidden: HashSet::new(),
            output: Box::into_pin(Box::new(array::from_fn::<_, O, _>(|_| Output::new()))),
            conns: BTreeSet::new(),
            fitness: f32::default(),
        }
    }

    pub fn mutate_add_conn(&mut self, rng: &mut impl Rng) {
        let leading = self.input.iter().map(Leading::from)
            .chain(self.hidden.iter().map(|hidden| Leading::from(hidden.as_ref())))
            .choose(rng).unwrap();

        let trailing = self.hidden.iter().map(|hidden| Trailing::from(hidden.as_ref()))
            .chain(self.output.iter().map(Trailing::from))
            .filter(|trailing| *trailing != leading)
            .choose(rng).unwrap();

        self.conns.insert(Conn::new(leading, trailing));
    }

    pub fn mutate_split_conn(&mut self, rng: &mut impl Rng) {
        let conn = self.conns.iter().filter(|conn| conn.enabled()).choose(rng).unwrap();
        conn.disable();

        let middle = self.hidden.get_or_insert(Hidden::new(conn)).as_ref();

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

