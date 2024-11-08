extern crate alloc;
use crate::{conn::*, node::*};
use core::{array, cell::RefCell, fmt, mem};
use alloc::{boxed::Box, rc::Rc, vec::Vec};
use hashbrown::{HashMap, HashSet};
use rand::{Rng, seq::IteratorRandom};

pub struct Genome<const I: usize, const O: usize> {
    pub inputs: Box<[Rc<Input>; I]>,
    pub hiddens: HashSet<Rc<Hidden>>,
    pub outputs: Box<[Rc<Output>; O]>,
    pub conns: Conns,
    pub fitness: f32,
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
            inputs: Box::new(array::from_fn::<_, I, _>(|innov| Input::new(innov))),
            hiddens: HashSet::new(),
            outputs: Box::new(array::from_fn::<_, O, _>(|innov| Output::new(I + innov))),
            conns: Conns::new(),
            fitness: f32::default(),
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
            .choose_stable(rng).unwrap();

        let trailing = self.hiddens.iter().map(Trailing::from)
            .chain(self.outputs.iter().map(Trailing::from))
            .filter(|trailing| *trailing != leading)
            .choose_stable(rng).unwrap();

        let conn = Conn::new(leading, trailing);
        self.conns.insert(conn);
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
        let conn = self.conns.iter_unordered()
            .filter(|conn| conn.enabled.get())
            .choose_stable(rng)
            .unwrap();

        conn.enabled.set(false);

        let middle = self.hiddens.get_or_insert(Hidden::new(conn));

        let first = Conn::new(&conn.leading, middle);
        let last = Conn::new(middle, &conn.trailing);

        self.conns.insert(first);
        self.conns.insert(last);
    }

    pub fn mutate_weight(&mut self) {
        todo!()
    }

    /// Evaluates the [`Genome`] by propagating the set of `inputs` up the `Genome`'s layers of nodes, modifying the
    /// them using weights and biases as they go.
    pub fn activate(&self, inputs: [f32; I]) -> [f32; O] {
        let mut map = HashMap::new();

        for conn in self.conns.iter_ordered().take_while(|conn| conn.enabled.get()) {
            let eval = match conn.leading {
                Leading::Input(ref input) => input.eval(conn.weight, inputs),
                Leading::Hidden(ref hidden) => hidden.eval(conn.weight, &mut map),
            };

            map.entry(conn.trailing.clone()).or_insert(Accum::new()).push(eval);
        }

        array::from_fn::<_, O, _>(|idx| self.outputs[idx].eval(&mut map))
    }

    pub fn compat_dist(&self) -> f32 {
        todo!()
    }

    pub fn crossover(mut lhs: Self, mut rhs: Self, rng: &mut impl Rng) -> Self {
        const MATCHING_PREF: f64 = 2.0 / 3.0;
        let rng = RefCell::new(rng);

        if lhs.fitness > rhs.fitness {
            mem::swap(&mut lhs, &mut rhs);
        }
        
        let mut inputs = Vec::with_capacity(I);
        let mut hiddens = HashSet::with_capacity(lhs.hiddens.len() + rhs.hiddens.len());
        let mut outputs = Vec::with_capacity(O);

        let matching = lhs.conns.hash_intersection(&rhs.conns).map(|key| {
            let choice = match lhs.fitness == rhs.fitness {
                false => rng.borrow_mut().gen_bool(MATCHING_PREF),
                true => rng.borrow_mut().gen(),
            };

            let parent = match choice {
                false => &lhs,
                true => &rhs,
            };

            parent.conns.get(key)
        });

        let disjoint: Box<dyn Iterator<Item = &Conn>> = match lhs.fitness == rhs.fitness { // use == in release
            false => Box::new(rhs.conns.hash_difference(&lhs.conns)),
            true => Box::new(lhs.conns
                .hash_symmetric_difference(&rhs.conns)
                .filter_map(|conn| rng.borrow_mut().gen::<bool>().then_some(conn))),
        };

        let conns = Conns::from_conns_iter(matching.chain(disjoint));

        for conn in conns.iter_ordered() { // use unordered after debugging
            dbg!(&conn.trailing);

            match conn.leading {
                Leading::Input(ref input) => {
                    let new = Rc::new(Input::clone(input));
                    inputs.push(new);
                }
                Leading::Hidden(ref hidden) => {
                    let new = Rc::new(Hidden::clone(hidden));
                    let inserted = hiddens.insert(new);
                    assert!(inserted);
                }
            }

            match conn.trailing {
                Trailing::Hidden(ref hidden) => {
                    let new = Rc::new(Hidden::clone(hidden));
                    let inserted = hiddens.insert(new);
                    assert!(inserted);
                }
                Trailing::Output(ref output) => {
                    let new = Rc::new(Output::clone(output));
                    outputs.push(new);
                }
            }
        }
        
        // TODO: Update idx for inputs.
        Self {
            inputs: inputs.try_into().unwrap(),
            hiddens,
            outputs: outputs.try_into().unwrap(),
            conns,
            fitness: f32::default(),
        }
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
            .field_with("conns", |f| f.debug_list().entries(self.conns.iter_ordered()).finish())
            .finish()
    }
}

