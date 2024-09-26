use crate::{conn::Conn, node::*};
use std::collections::{BTreeSet, HashSet};
use rand::{Rng, seq::IteratorRandom};

pub(crate) struct Genome<'g, const INPUTS: usize, const OUTPUTS: usize> {
    conns: BTreeSet<Conn<'g>>,
    input: [Input<'g>; INPUTS],
    hidden: HashSet<Hidden<'g>>,
    output: [Output; OUTPUTS],
    fitness: f32,
}

impl<'g, const INPUTS: usize, const OUTPUTS: usize> Genome<'g, INPUTS, OUTPUTS> {
    fn iter_conn_inputs(&'g self) -> impl Iterator<Item = ConnInput<'g>> {
        self.input.iter().map(|input| input.into()).chain(self.hidden.iter().map(|hidden| hidden.into()))
    }

    fn iter_conn_outputs(&'g self) -> impl Iterator<Item = ConnOutput<'g>> {
        self.hidden.iter().map(|hidden| hidden.into()).chain(self.output.iter().map(|output| output.into()))
    }

    fn mutate_add_conn(&'g mut self, rng: &mut impl Rng) {
        let rand_input = self.iter_conn_inputs().choose(rng).unwrap();
    }

    fn mutate_split_conn(&'g mut self, rng: &mut impl Rng) {
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

    fn activate(&mut self, inputs: [f32; INPUTS]) -> [f32; OUTPUTS] {
        todo!()
    }
}
