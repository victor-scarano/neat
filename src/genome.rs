use crate::{conn::Conn, node::{Node, ConnInput, ConnOutput, Hidden, Input, Output}};
use std::{cell::RefCell, collections::{BTreeSet, HashSet}};
use rand::{Rng, seq::IteratorRandom};

pub(crate) struct Genome<'g> {
    conns: BTreeSet<RefCell<Conn<'g>>>,
    input: Box<[Input<'g>]>,
    hidden: HashSet<Hidden<'g>>,
    output: Box<[Output]>,
    fitness: f32,
}

impl<'g> Genome<'g> {
    fn insert_conn(&mut self, conn: Conn<'g>) -> &RefCell<Conn<'g>> {
        let conn = RefCell::new(conn);
        self.conns.insert(conn.clone());
        self.conns.get(&conn).unwrap()
    }

    fn iter_conns(&self) -> impl Iterator<Item = &RefCell<Conn<'g>>> {
        self.conns.iter()
    }

    fn insert_node(&mut self, _node: Hidden<'g>) -> &Hidden<'g> {
        // self.hidden.insert(node.clone());
        // self.hidden.get(&node).unwrap();
        self.hidden.iter().nth(0).unwrap()
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
        let (input, output) = {
            let mut split_conn = self.iter_conns()
                .filter(|conn| conn.borrow().enabled())
                .choose(rng)
                .unwrap()
                .borrow_mut();

            split_conn.disable();

            (split_conn.input(), split_conn.output())
        };

        /*
        let mid_node = *self.insert_node(Hidden::new(rng));

        let initial_node = self.insert_conn(Conn::new(input, self.hidden.get(&mid_node).unwrap()));
        let final_node = self.insert_conn(Conn::new(self.hidden.get(&mid_node).unwrap(), output));

        mid_node.insert_conn(initial_node);
        mid_node.insert_conn(initial_node);
        */
    }
}
