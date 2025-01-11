extern crate alloc;
use crate::{edge::*, node::*};
use core::{array, fmt, mem};
use alloc::{collections::BTreeMap, vec::Vec, rc::Rc};
use bumpalo::Bump;
use hashbrown::{HashMap, HashSet};
use rand::{Rng, seq::IteratorRandom};

#[derive(Clone, Debug)]
pub struct Genome<const I: usize, const O: usize> {
    pub inputs: Inputs<I>,
    pub outputs: Outputs<O>,
    pub hiddens: Hiddens,
    pub edges: Edges,
    pub fitness: f32,
}

impl<const I: usize, const O: usize> Genome<I, O> {
    pub fn new() -> Self {
        assert_ne!(I, 0);
        assert_ne!(O, 0);

        Self {
            inputs: Inputs::new(),
            outputs: Outputs::new::<I>(),
            hiddens: Hiddens::new(),
            edges: Edges::new(),
            fitness: 0.0
        }
    }

    pub fn mutate_add_edge(&mut self, rng: &mut impl Rng) {
        let tail = self.inputs.iter().map(Tail::from)
            .chain(self.hiddens.iter().map(Tail::from))
            .choose_stable(rng).unwrap();

        let head = self.hiddens.iter().map(Head::from)
            .chain(self.outputs.iter().map(Head::from))
            .filter(|head| tail != *head) // check for ptr eq
            .choose_stable(rng).unwrap();

        let edge = Edge::new(tail, head);
        self.edges.insert(edge);
    }

    pub fn mutate_split_edge(&mut self, rng: &mut impl Rng) {
        let edge = self.edges.iter()
            .filter(|edge| edge.enabled.get())
            .choose_stable(rng).unwrap();

        edge.enabled.set(false);

        let (first, last) = self.hiddens.split_edge(edge);
        self.edges.insert(first);
        self.edges.insert(last);
    }

    pub fn mutate_weight(&mut self) {
        todo!()
    }

    pub fn activate(&self, inputs: [f32; I]) -> [f32; O] {
        let mut map = HashMap::new();

        // needs to iter in ordered
        for edge in self.edges.iter().take_while(|edge| edge.enabled.get()) {
            let eval = match edge.tail() {
                Tail::Input(input) => input.eval(edge.weight, inputs),
                Tail::Hidden(hidden) => hidden.eval(edge.weight, &mut map),
            };

            map.entry(edge.head()).or_insert(Accum::new()).push(eval);
        }

        array::from_fn::<_, O, _>(|idx| self.outputs.eval_nth(idx, &mut map))
    }

    pub fn compat_dist(&self) -> f32 {
        todo!()
    }

    pub fn crossover(lhs: Self, rhs: Self, rng: &mut impl Rng) -> Self {
        todo!()
    }
}

