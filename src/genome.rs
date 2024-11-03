extern crate alloc;
use crate::{conn::Conn, node::*};
use core::{array, fmt};
use alloc::{boxed::Box, collections::BTreeSet, rc::Rc};
use hashbrown::{HashMap, HashSet};
use rand::{Rng, seq::IteratorRandom};

pub struct Genome<const I: usize, const O: usize> {
    pub inputs: Box<[Rc<Input>; I]>,
    pub hiddens: HashSet<Rc<Hidden>>,
    pub outputs: Box<[Rc<Output>; O]>,
    pub conns: BTreeSet<Conn>,
}

impl<const I: usize, const O: usize> Genome<I, O> {
    /// Creates a new [`Genome`].
    ///
    /// # Panics
    /// Panics if either `Genome::I` or `Genome::O` is 0.
    pub fn new() -> Self {
        assert_ne!(I, 0);
        assert_ne!(O, 0);

        Self {
            inputs: Box::new(array::from_fn::<_, I, _>(|idx| Input::new(idx))),
            hiddens: HashSet::new(),
            outputs: Box::new(array::from_fn::<_, O, _>(|_| Output::new())),
            conns: BTreeSet::new(),
        }
    }

    /// Creates a [`Conn`] out of two previously unconnected nodes and inserts it into the [`Genome`].
    ///
    /// # Panics
    /// - If there are no [`Leading`] nodes in the `Genome`
    /// - If there are no [`Trailing`] nodes in the `Genome` that aren't the same as the selected `Leading` node.
    /// - If the new `Conn` was not inserted.
    pub fn mutate_add_conn(&mut self, rng: &mut impl Rng) {
        let leading = self.inputs.iter().map(Leading::from)
            .chain(self.hiddens.iter().map(Leading::from))
            .choose_stable(rng)
            .unwrap();

        let trailing = self.hiddens.iter().map(Trailing::from)
            .chain(self.outputs.iter().map(Trailing::from))
            .filter(|trailing| *trailing != leading)
            .choose_stable(rng)
            .unwrap();

        let conn = Conn::new(leading, trailing);
        let inserted = self.conns.insert(conn);
        assert!(inserted);
    }

    /// As per [Stanley's paper](https://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf), an existing [`Conn`] is
    /// split into two new ones, joined by a new node, all of which are inserted into the [`Genome`]. The old `Conn` is
    /// disabled, the `Conn` leading into the new node recieves a weight of 1, and the `Conn` leading out of the new
    /// node receives the same weight as the old `Conn`.
    ///
    /// # Panics
    /// - If there are no enabled [`Conn`]s to split from.
    /// - If the first new `Conn` was not inserted.
    /// - If the second new `Conn` was not inserted.
    pub fn mutate_split_conn(&mut self, rng: &mut impl Rng) {
        let conn = self.conns.iter()
            .filter(|conn| conn.enabled.get())
            .choose(rng)
            .unwrap();

        conn.enabled.set(false);

        let middle = self.hiddens.get_or_insert(Hidden::new(conn));

        let first = Conn::new(&conn.leading, middle);
        let last = Conn::new(middle, &conn.trailing);

        let inserted = self.conns.insert(first);
        assert!(inserted);

        let inserted = self.conns.insert(last);
        assert!(inserted);
    }

    pub fn mutate_weight(&mut self) {
        todo!()
    }

    pub fn activate(&self, inputs: [f32; I]) -> [f32; O] {
        let mut map = HashMap::new();

        for layer in self.conns.iter().filter(|conn| conn.enabled.get()) {
            let eval = match layer.leading {
                Leading::Input(ref input) => input.eval(layer, inputs),
                Leading::Hidden(ref hidden) => hidden.eval(layer, &mut map),
            };

            map.entry(layer.trailing.clone()).or_insert(Accum::new()).push(eval);
        }

        array::from_fn::<_, O, _>(|idx| self.outputs.get(idx).unwrap().eval(&mut map))
    }

    pub fn compat_dist(&self) -> f32 {
        todo!()
    }

    pub fn crossover(lhs: Self, rhs: Self) -> Self {
        todo!()
    }
}

impl<const I: usize, const G: usize> fmt::Debug for Genome<I, G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f
            .debug_struct("Genome")
            .field_with("inputs", |f| self.inputs.iter().fold(&mut f.debug_map(), |f, input| {
                f.key_with(|f| fmt::Pointer::fmt(input, f)).value(input)
            }).finish())
            .field_with("hiddens", |f| self.hiddens.iter().fold(&mut f.debug_map(), |f, hidden| {
                f.key_with(|f| fmt::Pointer::fmt(hidden, f)).value(hidden)
            }).finish())
            .field_with("outputs", |f| self.outputs.iter().fold(&mut f.debug_map(), |f, output| {
                f.key_with(|f| fmt::Pointer::fmt(output, f)).value(output)
            }).finish())
            .field_with("conns", |f| f.debug_list().entries(self.conns.iter()).finish())
            .finish()
    }
}

