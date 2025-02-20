extern crate alloc;
use crate::{edge::*, fitness::Fitness, node::*};
use core::array;
use alloc::boxed::Box;
use hashbrown::{HashMap, HashSet};
use rand::{Rng, seq::IteratorRandom};

#[derive(Debug)]
pub struct Genome<'a, const I: usize, const O: usize> {
    pub inputs: Box<[Input; I]>,
    pub outputs: Box<[Output; O]>,
    pub hiddens: Hiddens,
    pub edges: Edges<'a>,
    pub fitness: Fitness,
}

impl<'a, const I: usize, const O: usize> Genome<'a, I, O> {
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
            let eval = match edge.tail {
                Tail::Input(input) => input.eval(edge.weight, inputs),
                Tail::Hidden(hidden) => hidden.eval(edge.weight, &mut map),
            };

            map.entry(&edge.head).or_insert(Accum::new()).push(eval);
        }

        array::from_fn::<_, O, _>(|idx| self.outputs.get(idx).unwrap().eval(&mut map))
    }

    pub fn compat_dist(&self) -> f32 {
        todo!()
    }

    pub fn crossover(lhs: Self, rhs: Self, rng: &mut impl Rng) -> Self {
        // choose edges for child that will be inherited from parents
        // let int = Edges::innov_matching(&lhs, &rhs, rng);
        // let diff = Edges::innov_disjoint(&lhs, &rhs, rng);

        let tails = HashSet::<Tail>::new();
        let heads = HashSet::<Head>::new();

        todo!()
    }
}

impl<const I: usize, const O: usize> Default for Genome<'_, I, O> {
    fn default() -> Self {
        assert_ne!(I, 0);
        assert_ne!(O, 0);

        Self {
            inputs: Box::new(array::from_fn::<_, I, _>(Input::new)),
            outputs: Box::new(array::from_fn::<_, O, _>(Output::new::<I>)),
            hiddens: Hiddens::default(),
            edges: Edges::default(),
            fitness: Fitness::default(),
        }
    }
}
