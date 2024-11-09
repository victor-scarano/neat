extern crate alloc;
use crate::{conn::*, node::*};
use core::{array, cmp, fmt, mem, ptr};
use alloc::{borrow::Cow, boxed::Box, rc::*, vec::Vec};
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
    pub fn new() -> Self {
        assert_ne!(I, 0);
        assert_ne!(O, 0);

        Self {
            inputs: Box::new(array::from_fn::<_, I, _>(|innov| Input::new(innov))),
            hiddens: HashSet::default(),
            outputs: Box::new(array::from_fn::<_, O, _>(|innov| Output::new(I + innov))),
            conns: Conns::default(),
            fitness: f32::default(),
        }
    }

    pub fn mutate_add_conn(&mut self, rng: &mut impl Rng) -> Weak<Conn> {
        let leading = self.inputs.iter().map(Leading::from)
            .chain(self.hiddens.iter().map(Leading::from))
            .choose_stable(rng).unwrap();

        let trailing = self.hiddens.iter().map(Trailing::from)
            .chain(self.outputs.iter().map(Trailing::from))
            .filter(|trailing| *trailing != leading)
            .choose_stable(rng).unwrap();

        let conn = Conn::new(leading, trailing);
        self.conns.insert(conn)
    }

    pub fn mutate_split_conn(&mut self, rng: &mut impl Rng) -> (Weak<Conn>, Weak<Hidden>, Weak<Conn>) {
        // iter ordered here to ensure that the randomly chosen conn is consistent
        let conn = self.conns.iter_ordered()
            .filter(|conn| conn.enabled.get())
            .choose_stable(rng).unwrap();

        conn.enabled.set(false);

        // must always insert, but cant check to make sure it inserted
        // ideally we want an insert_and_get -> Option<&Hidden> so we can check
        let middle = self.hiddens.get_or_insert(Hidden::new(conn));

        let first = Conn::new(&conn.leading, middle);
        let last = Conn::new(middle, &conn.trailing);

        let first = self.conns.insert(first);
        let middle = Rc::downgrade(middle);
        let last = self.conns.insert(last);

        (first, middle, last)
    }

    pub fn mutate_weight(&mut self) {
        todo!()
    }

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

        if lhs.fitness > rhs.fitness {
            mem::swap(&mut lhs, &mut rhs);
        }
        
        let mut inputs = Vec::with_capacity(I);
        let mut hiddens = HashSet::with_capacity(lhs.hiddens.len() + rhs.hiddens.len());
        let mut outputs = Vec::with_capacity(O);

        let mut matching = Vec::with_capacity(cmp::max(lhs.conns.len(), rhs.conns.len()));
        lhs.conns.hash_intersection(&rhs.conns).map(|key| {
            let choice = match lhs.fitness == rhs.fitness {
                false => rng.gen_bool(MATCHING_PREF),
                true => rng.gen(),
            };

            let parent = match choice {
                false => &lhs,
                true => &rhs,
            };

            parent.conns.get(key)
        }).collect_into(&mut matching);

        let mut disjoint = Vec::with_capacity(lhs.conns.len() + rhs.conns.len());
        match lhs.fitness == rhs.fitness {
            false => rhs.conns.hash_difference(&lhs.conns).collect_into(&mut disjoint),
            true => lhs.conns.hash_symmetric_difference(&rhs.conns).filter(|_| rng.gen()).collect_into(&mut disjoint),
        };

        for conn in matching.iter().chain(disjoint.iter()) { // use unordered after debugging
            // dbg!(&conn.trailing);

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
            conns: Conns::from(matching, disjoint),
            fitness: f32::default(),
        }
    }
}

impl<const I: usize, const O: usize> Clone for Genome<I, O> {
    fn clone(&self) -> Self {
        let inputs = Box::new(self.inputs.clone().map(Rc::unwrap_or_clone).map(Rc::new));
        let hiddens = self.hiddens.iter().cloned().map(Rc::unwrap_or_clone).map(Rc::new).collect();
        let outputs = Box::new(self.outputs.clone().map(Rc::unwrap_or_clone).map(Rc::new));
        let conns = self.conns.clone_from(&inputs, &hiddens, &outputs);
        Self { inputs, hiddens, outputs, conns, fitness: self.fitness }
    }
}

impl<const I: usize, const O: usize> fmt::Debug for Genome<I, O> {
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

impl<'a, const I: usize, const O: usize> dot::GraphWalk<'a, usize, (usize, usize)> for Genome<I, O> {
    fn nodes(&'a self) -> dot::Nodes<'a, usize> {
        Cow::Owned(self.inputs.iter().map(|input| ptr::addr_of!(*input.as_ref()) as usize)
            .chain(self.hiddens.iter().map(|hidden| ptr::addr_of!(*hidden.as_ref()) as usize))
            .chain(self.outputs.iter().map(|output| ptr::addr_of!(*output.as_ref()) as usize))
            .collect())
    }

    fn edges(&'a self) -> dot::Edges<'a, (usize, usize)> {
        Cow::Owned(self.conns.iter_ordered().map(|conn| {
            let leading = match conn.leading {
                Leading::Input(ref input) => ptr::addr_of!(*input.as_ref()) as usize,
                Leading::Hidden(ref hidden) => ptr::addr_of!(*hidden.as_ref()) as usize,
            };

            let trailing = match conn.trailing {
                Trailing::Hidden(ref hidden) => ptr::addr_of!(*hidden.as_ref()) as usize,
                Trailing::Output(ref output) => ptr::addr_of!(*output.as_ref()) as usize,
            };

            (leading, trailing)
        }).collect())
    }
}
