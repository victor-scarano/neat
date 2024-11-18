extern crate alloc;
use crate::{conn::*, node::*};
use core::{array, cmp, fmt, mem};
use alloc::{boxed::Box, collections::BTreeMap, rc::*, vec::Vec};
use hashbrown::{HashMap, HashSet};
use rand::{Rng, seq::IteratorRandom};

pub struct Genome<'g, const I: usize, const O: usize> {
    pub inputs: Inputs<I>,
    pub hiddens: Hiddens,
    pub outputs: Outputs<I, O>,
    pub conns: Conns<'g>,
    pub fitness: f32,
}

impl<const I: usize, const O: usize> Genome<'_, I, O> {
    pub fn new() -> Self {
        assert_ne!(I, 0);
        assert_ne!(O, 0);

        Self {
            inputs: Inputs::new(),
            hiddens: Hiddens::new(),
            outputs: Outputs::new(),
            conns: Conns::default(),
            fitness: f32::default(),
        }
    }

    pub fn mutate_add_conn(&mut self, rng: &mut impl Rng) -> Weak<Conn> {
        let tail = self.inputs.iter().map(Tail::from)
            .chain(self.hiddens.iter().map(Tail::from))
            .choose_stable(rng).unwrap();

        let head = self.hiddens.iter().map(Head::from)
            .chain(self.outputs.iter().map(Head::from))
            .filter(|head| *head != tail)
            .choose_stable(rng).unwrap();

        let conn = Conn::new(tail, head);
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

        let first = Conn::new(&conn.tail, middle);
        let last = Conn::new(middle, &conn.head);

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
            let eval = match conn.tail {
                Tail::Input(ref input) => input.eval(conn.weight, inputs),
                Tail::Hidden(ref hidden) => hidden.eval(conn.weight, &mut map),
            };

            map.entry(conn.head.clone()).or_insert(Accum::new()).push(eval);
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
            // dbg!(&conn.head);

            match conn.tail {
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

            match conn.head {
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

// probably a better way to do this but it works for now lmao
// TODO: makesure dot formatting is correct
// TODO: add input/output arrow indicators like in stanleys paper
// TODO: sometimes nodes go out of order in their subgraph
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

        for conn in self.conns.iter_ordered() {
            write!(f, "\t")?;

            if !conn.enabled.get() {
                write!(f, "// ")?;
            }

            writeln!(f, "N{} -> N{}", conn.tail.innov(), conn.head.innov())?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}

