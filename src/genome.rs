extern crate alloc;
use crate::{edge::*, node::*};
use core::{array, fmt, mem};
use alloc::{collections::BTreeMap, vec::Vec, rc::Rc};
use hashbrown::{HashMap, HashSet};
use rand::{Rng, seq::IteratorRandom};

#[derive(Debug)]
pub struct Genome<const I: usize, const O: usize> {
    pub bump: Bump,
    pub inputs: Inputs<I>,
    pub outputs: Outputs<O>,
    pub edges: Edges,
    pub fitness: f32,
}

impl<const I: usize, const O: usize> Genome<I, O> {
    pub fn new() -> Self {
        assert_ne!(I, 0);
        assert_ne!(O, 0);

        let bump = Bump::new();
        let inputs = Inputs::new_in(bump.clone());
        let outputs = Outputs::new_in::<I>(bump.clone());

        Self { bump, inputs, outputs, edges: Edges::new(), fitness: 0.0 }
    }

    pub fn mutate_add_edge(&mut self, rng: &mut impl Rng) {
        let (first, second) = self.edges.random_edges(rng);
        // TODO: allow for any non-equal node combo from the random edges to be used
        let edge = Edge::new(first.tail.clone(), second.head.clone());
        self.edges.insert(edge);
    }

    pub fn mutate_split_edge(&mut self, rng: &mut impl Rng) {
        let edge = self.edges.iter_ordered().filter(|edge| edge.enabled.get()).choose_stable(rng).unwrap();
        edge.enabled.set(false);
        let (first, last) = edge.split();
        self.edges.insert(first);
        self.edges.insert(last);
    }

    pub fn mutate_weight(&mut self) {
        todo!()
    }

    pub fn activate(&self, inputs: [f32; I]) -> [f32; O] {
        let mut map = HashMap::new();

        for edge in self.edges.iter_ordered().take_while(|edge| edge.enabled.get()) {
            let eval = match edge.tail {
                Tail::Input(ref input) => input.eval(edge.weight, inputs),
                Tail::Hidden(ref hidden) => hidden.eval(edge.weight, &mut map),
            };

            map.entry(edge.head.clone()).or_insert(Accum::new()).push(eval);
        }

        array::from_fn::<_, O, _>(|idx| self.outputs.eval_nth(idx, &mut map))
    }

    pub fn compat_dist(&self) -> f32 {
        todo!()
    }

    pub fn crossover(lhs: Self, rhs: Self, rng: &mut impl Rng) -> Self {
        Edges::crossover(lhs, rhs, rng)
    }
}

