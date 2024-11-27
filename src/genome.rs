extern crate alloc;
use crate::{edge::*, node::*};
use core::{array, fmt, marker::PhantomPinned, pin::Pin};
use alloc::{collections::BTreeMap, vec::Vec, rc::Rc};
use bumpalo::Bump;
use hashbrown::HashMap;
use rand::{Rng, seq::IteratorRandom};

#[derive(Debug)]
pub struct Genome<'genome, const I: usize, const O: usize> {
    pub inputs: Inputs<'genome, I>,
    pub outputs: Outputs<'genome, O>,
    pub edges: Edges<'genome>,
    pub fitness: f32,
}

impl<'genome, const I: usize, const O: usize> Genome<'genome, I, O> {
    pub fn new_in(bump: &'genome Bump) -> Self {
        assert_ne!(I, 0);
        assert_ne!(O, 0);

        let inputs = Inputs::new_in(bump);
        let outputs = Outputs::new_in::<I>(bump);

        Self {
            inputs,
            outputs,
            edges: Edges::new(),
            fitness: 0.0,
        }
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

    pub fn mutate_weight(&self) {
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

    /*
    pub fn crossover(mut lhs: Self, mut rhs: Self, rng: &mut impl Rng) -> Self {
        const MATCHING_PREF: f64 = 2.0 / 3.0;

        if lhs.fitness > rhs.fitness {
            mem::swap(&mut lhs, &mut rhs);
        }
        
        let mut inputs = Vec::with_capacity(I);
        let mut hiddens = HashSet::with_capacity(lhs.hiddens.len() + rhs.hiddens.len());
        let mut outputs = Vec::with_capacity(O);

        let mut matching = Vec::with_capacity(cmp::max(lhs.edges.len(), rhs.edges.len()));
        lhs.edges.hash_intersection(&rhs.edges).map(|key| {
            let choice = match lhs.fitness == rhs.fitness {
                false => rng.gen_bool(MATCHING_PREF),
                true => rng.gen(),
            };

            let parent = match choice {
                false => &lhs,
                true => &rhs,
            };

            parent.edges.get(key)
        }).collect_into(&mut matching);

        let mut disjoint = Vec::with_capacity(lhs.edges.len() + rhs.edges.len());
        match lhs.fitness == rhs.fitness {
            false => rhs.edges.hash_difference(&lhs.edges).collect_into(&mut disjoint),
            true => lhs.edges.hash_symmetric_difference(&rhs.edges).filter(|_| rng.gen()).collect_into(&mut disjoint),
        };

        for edge in matching.iter().chain(disjoint.iter()) { // use unordered after debugging
            // dbg!(&edge.head);

            match edge.tail {
                Tail::Input(ref input) => {
                    let new = Rc::new(Input::clone(input));
                    inputs.push(new);
                }
                Tail::Hidden(ref hidden) => {
                    let new = Rc::new(Hidden::clone(hidden));
                    let inserted = hiddens.insert(new);
                    assert!(inserted);
                }
            }

            match edge.head {
                Head::Hidden(ref hidden) => {
                    let new = Rc::new(Hidden::clone(hidden));
                    let inserted = hiddens.insert(new);
                    assert!(inserted);
                }
                Head::Output(ref output) => {
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
            edges: Edges::from(matching, disjoint),
            fitness: f32::default(),
        }
    }
    */
}

// probably a better way to do this but it works for now lmao
// sometimes nodes go out of order in their subgraph
/*
impl<const I: usize, const O: usize> fmt::Display for Genome<I, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "digraph genome {{")?;

        writeln!(f, "\t// nodesep = 0.3")?;
        writeln!(f, "\trank = same")?;
        writeln!(f, "\trankdir = BT")?;
        writeln!(f, "\t// ranksep = 0.2")?;
        writeln!(f, "")?;

        writeln!(f, "\tnode [fixedsize = true]")?;
        writeln!(f, "\tnode [fontsize = 7]")?;
        writeln!(f, "\tnode [shape = circle]")?;
        writeln!(f, "\tnode [style = filled]")?;
        writeln!(f, "\tnode [width = 0.15]")?;
        writeln!(f, "")?;

        let mut map = BTreeMap::<_, Vec<_>>::new();

        for input in self.inputs.iter() {
            map.entry(input.layer()).or_default().push(input.innov());
        }

        for hidden in self.hiddens.iter() {
            map.entry(hidden.layer()).or_default().push(hidden.innov());
        }

        for output in self.outputs.iter() {
            map.entry(output.layer()).or_default().push(output.innov());
        }

        while let Some((layer, innovs)) = map.pop_first() {
            writeln!(f, "\tsubgraph {} {{", layer)?;
            
            for innov in innovs {
                writeln!(f, "\t\tN{} [label = {}]", innov, innov + 1)?;
            }

            writeln!(f, "\t}}")?;
            writeln!(f, "")?;
        }

        writeln!(f, "\tedge [arrowsize = 0.3]")?;
        writeln!(f, "")?;

        for edge in self.edges.iter_ordered() {
            write!(f, "\t")?;

            if !edge.enabled.get() {
                write!(f, "// ")?;
            }

            writeln!(f, "N{} -> N{}", edge.tail.innov(), edge.head.innov())?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}
*/
