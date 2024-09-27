use crate::{conn::Conn, node::*};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use rand::{Rng, seq::IteratorRandom};

pub(crate) struct Genome<'genome, const INPUTS: usize, const OUTPUTS: usize> {
    conns: BTreeSet<Conn<'genome>>,
    input: [Input<'genome>; INPUTS],
    hidden: HashSet<Hidden<'genome>>,
    output: [Output; OUTPUTS],
    fitness: f32,
}

impl<'genome, const INPUTS: usize, const OUTPUTS: usize> Genome<'genome, INPUTS, OUTPUTS> {
    fn mutate_add_conn(&'genome mut self, rng: &mut impl Rng) {
        let input: ConnInput = self.input.iter().map(|input| input.into())
            .chain(self.hidden.iter().map(|hidden| hidden.into()))
            .choose(rng).unwrap();

        let output = self.hidden.iter().map(|hidden| hidden.into())
            .chain(self.output.iter().map(|output| output.into()))
            .filter(|node| *node != input)
            .choose(rng).unwrap();

        let conn = Conn::new(input.clone(), output);
        self.conns.insert(conn.clone());
        let conn = self.conns.get(&conn).unwrap();
        
        input.insert_forward_conn(conn);
    }

    fn mutate_split_conn(&'genome mut self, rng: &mut impl Rng) {
        let conn = self.conns.iter().filter(|conn| conn.enabled()).choose(rng).unwrap();
        conn.disable();

        let node = Hidden::new(rng);
        self.hidden.insert(node.clone());
        let middle = self.hidden.get(&node).unwrap();

        let initial = Conn::new(conn.input(), middle.into());
        let r#final = Conn::new(middle.into(), conn.output());

        self.conns.insert(initial.clone());
        let initial = self.conns.get(&initial).unwrap();

        self.conns.insert(r#final.clone());
        let r#final = self.conns.get(&r#final).unwrap();
    }

    fn mutate_weight(&mut self, rng: &mut impl Rng) {
        todo!();
    }

    fn activate(&self, inputs: [f32; INPUTS]) -> [f32; OUTPUTS] {
        let mut map = BTreeMap::<ConnOutput, f32>::new();

        for (node, input) in self.input.iter().zip(inputs.iter()) {
            for conn in node.forward_conns().iter().filter(|conn| conn.enabled()) {
                *map.entry(conn.output()).or_default() += input * conn.weight();
            }
        }

        todo!();
    }

    fn compat_dist(&self) -> f32 {
        todo!();
    }

    fn crossover(lhs: Self, rhs: Self) -> Self {
        todo!();
    }
}
