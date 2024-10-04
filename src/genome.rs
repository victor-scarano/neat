use crate::{conn::Conn, node::*};
use std::{array, collections::*, fmt};
use rand::{Rng, seq::IteratorRandom};

pub struct Genome<'genome, const INPUTS: usize, const OUTPUTS: usize, R: Rng> {
    conns: BTreeSet<Conn<'genome>>,
    input: Box<[Input; INPUTS]>,
    hidden: HashSet<Hidden>,
    output: Box<[Output; OUTPUTS]>,
    fitness: f32,
    rng: R,
}

impl<'genome, const INPUTS: usize, const OUTPUTS: usize, R: Rng> Genome<'genome, INPUTS, OUTPUTS, R> {
    pub fn new(rng: R) -> Self {
        Self {
            conns: BTreeSet::new(),
            input: array::from_fn::<_, INPUTS, _>(|idx| Input::new()).into(),
            hidden: HashSet::new(),
            output: array::from_fn::<_, OUTPUTS, _>(|idx| Output::new()).into(),
            fitness: f32::default(),
            rng,
        }
    }

    pub fn mutate_add_conn(&mut self) {
        let input: ConnInput = self.input.iter().map(|input| input.into())
            .chain(self.hidden.iter().map(|hidden| hidden.into()))
            .choose(&mut self.rng).unwrap();

        let output = self.hidden.iter().map(|hidden| hidden.into())
            .chain(self.output.iter().map(|output| output.into()))
            .filter(|node| *node != input)
            .choose(&mut self.rng).unwrap();

        let conn = Conn::new(input.clone(), output);
        self.conns.insert(conn.clone());
    }

    pub fn mutate_split_conn(&'genome mut self) {
        let conn = self.conns.iter()
            .filter(|conn| conn.enabled())
            .choose(&mut self.rng).unwrap();
        conn.disable();

        let node = Hidden::new(conn);
        self.hidden.insert(node.clone());
        let middle = self.hidden.get(&node).unwrap();

        let initial = Conn::new(conn.conn_input(), middle.into());
        let r#final = Conn::new(middle.into(), conn.conn_output());

        self.conns.insert(initial.clone());
        let initial = self.conns.get(&initial).unwrap();

        self.conns.insert(r#final.clone());
        let r#final = self.conns.get(&r#final).unwrap();
    }

    pub fn mutate_weight(&mut self) {
        todo!();
    }

    pub fn activate(&self, inputs: [f32; INPUTS]) -> [f32; OUTPUTS] {
        // activation(bias + (response * aggregation(inputs)))
        // input nodes have: activation=identity, response=1, agreggation=none

        let mut map = BTreeMap::<_, f32>::new();

        // for (node, value) in self.input.iter().zip(inputs.iter()) {
            // for conn in node.conns().iter().filter(|conn| conn.enabled()) {
                // *map.entry(conn.conn_output()).or_default() += (node.bias() + value) * conn.weight();
            // }
        // }

        for (conn, conn_input) in self.conns.iter().filter_map(|conn| {
            conn.conn_input().hidden().map(|conn_input| (conn, conn_input))
        }) {
            let aggregated = map.get(&conn_input.into()).unwrap();
            *map.entry(conn.conn_output()).or_default() += 
                conn_input.activate(conn_input.bias() + (conn_input.response() * aggregated));
        }

        todo!();
    }

    pub fn compat_dist(&self) -> f32 {
        todo!();
    }

    pub fn crossover(lhs: Self, rhs: Self) -> Self {
        todo!();
    }
}

impl<const INPUTS: usize, const OUTPUTS: usize, R: Rng> fmt::Debug for Genome<'_, INPUTS, OUTPUTS, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Genome")
            .field_with("Conns", |f| f.debug_list().entries(self.conns.iter()).finish())
            .finish()
    }
}

