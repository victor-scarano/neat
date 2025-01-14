extern crate alloc;
use crate::{edge::*, fitness::Fitness, node::*};
use core::{array, mem};
use hashbrown::HashMap;
use rand::{Rng, seq::IteratorRandom};

#[derive(Clone, Debug)]
pub struct Genome<const I: usize, const O: usize> {
    pub inputs: Inputs<I>,
    pub outputs: Outputs<O>,
    pub hiddens: Hiddens,
    pub edges: Edges,
    pub fitness: Fitness,
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
            fitness: Fitness::from(0.0)
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

    pub fn crossover(mut lhs: Self, mut rhs: Self, rng: &mut impl Rng) -> Self {
        // swap to order by fitness
        if lhs.fitness > rhs.fitness {
            mem::swap(&mut lhs, &mut rhs);
        }

        let mut edges = Edges::new();

        Edges::innov_int(&lhs.edges, &rhs.edges).map(|key| {
            let choice = Fitness::gen_bool(lhs.fitness, rhs.fitness, rng);
            let parent = match choice { false => &lhs, true => &rhs };
            parent.edges.get(key).unwrap()
        }).collect_into(&mut edges);

        match lhs.fitness == rhs.fitness {
            true => Edges::innov_diff(&lhs.edges, &rhs.edges).collect_into(&mut edges),
            false => Edges::innov_sym_diff(&lhs.edges, &rhs.edges).filter(|_| rng.gen()).collect_into(&mut edges),
        };

        // at this point the child edges still point to the parent nodes.
        // in order to fix this, we need to store a map of the references to the
        // parent nodes to new child nodes.
        // we iterate over the child edges. if the node is already in the map,
        // replace the node in the edge with the new child node in the map. if
        // the node is not already in the map, insert the node as a key, and a
        // clone of it as the value, and set the value as the edge's node.
        // the only issue at this point is that i dont know how we're going to
        // actually mutate the child edges to replace the parent nodes that they
        // point to.
        for edge in edges.iter() {
            match edge.tail() {
                Tail::Input(input) => (),
                Tail::Hidden(hidden) => (),
            }

            match edge.head() {
                Head::Hidden(hidden) => (),
                Head::Output(output) => (),
            }
        }

        todo!()
    }
}

