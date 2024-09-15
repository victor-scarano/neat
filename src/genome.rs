use crate::{conn::Conn, node::{Node, ConnInput, ConnOutput}};
use std::collections::BTreeSet;
use rand::{Rng, seq::IteratorRandom};

pub(crate) struct Genome<'genome> {
    conns: BTreeSet<Conn<'genome>>,
    input: Box<[Box<dyn ConnInput<'genome>>]>,
    hidden: Vec<Box<dyn Node>>,
    output: Box<[Box<dyn ConnOutput<'genome>>]>,
    fitness: f32,
}

impl<'genome> Genome<'genome> {
    fn insert_conn(&mut self, conn: Conn<'genome>) -> &Conn<'genome> {
        self.conns.insert(conn.clone());
        self.conns.get(&conn).unwrap()
    }

    fn iter_conns(&self) -> impl Iterator<Item = &Conn<'genome>> {
        self.conns.iter()
    }

    fn iter_conn_inputs(&self) -> impl Iterator<Item = &dyn ConnInput<'genome>> {
        use std::ops::Deref;
        self.input.iter().map(|input| input.deref())
    }

    // fn iter_conn_outputs(&self) -> impl Iterator<Item = &dyn ConnOutput<'genome>> {}

    fn mutate_add_conn(&mut self, rng: &mut impl Rng) {
        let _rand_input = self.iter_conn_inputs().choose(rng).unwrap();
        // let rand_output = self.iter_conn_outputs().filter(|output| {
            // find conn output that can be downcasted as a hidden node
            // and that is not the same as the selected rand input
            // false
        // }).choose(rng).unwrap();

        // let new_conn = self.insert_conn(Conn::new(rand_input.clone(), rand_output.clone()));
    }
}
