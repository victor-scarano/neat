use crate::{conn::Conn, node::{Node, ConnInput, ConnOutput, Hidden, Input, Output}};
use std::collections::{BTreeSet, HashSet};
use rand::{Rng, seq::IteratorRandom};

pub(crate) struct Genome<'g> {
    conns: BTreeSet<Conn<'g>>,
    input: Box<[Input<'g>]>,
    hidden: HashSet<Hidden<'g>>,
    output: Box<[Output]>,
    fitness: f32,
}

impl<'g> Genome<'g> {
    fn insert_conn(&mut self, input: &'g dyn ConnInput<'g>, output: &'g dyn ConnOutput<'g>) -> &Conn<'g> {
        let conn = Conn::new(input, output);
        self.conns.insert(conn.clone());
        self.conns.get(&conn).unwrap()
    }

    fn iter_conns(&self) -> impl Iterator<Item = &Conn<'g>> {
        self.conns.iter()
    }

    fn insert_node(&mut self, node: Hidden<'g>) -> &Hidden<'g> {
        self.hidden.insert(node.clone());
        self.hidden.get(&node).unwrap()
    }

    fn iter_input(&self) -> impl Iterator<Item = &Input<'g>> {
        self.input.iter()
    }

    fn iter_hidden(&self) -> impl Iterator<Item = &Hidden<'g>> {
        self.hidden.iter()
    }

    fn iter_output(&self) -> impl Iterator<Item = &Output> {
        self.output.iter()
    }

    fn iter_conn_inputs(&self) -> impl Iterator<Item = &dyn ConnInput<'g>> {
        self.iter_input().map(|input| input as &dyn ConnInput)
            .chain(self.iter_hidden().map(|hidden| hidden as &dyn ConnInput))
    }

    fn iter_conn_outputs(&self) -> impl Iterator<Item = &dyn ConnOutput<'g>> {
        self.iter_hidden().map(|hidden| hidden as &dyn ConnOutput)
            .chain(self.iter_output().map(|output| output as &dyn ConnOutput))
    }

    fn mutate_add_conn(&mut self, _rng: &mut impl Rng) {
        todo!()
    }

    fn mutate_split_conn(&'g mut self, rng: &mut impl Rng) {
        let conn = self.iter_conns().filter(|conn| conn.enabled()).choose(rng).unwrap();

        conn.disable();

        let input = conn.input();
        let output = conn.output();

        let middle = self.insert_node(Hidden::new(rng));

        let initial = self.insert_conn(input, middle);
        let r#final = self.insert_conn(middle, output);

        // mid.insert_conn(initial);
        // mid.insert_conn(initial);
    }
}
