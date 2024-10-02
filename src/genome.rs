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
        // activation(bias + (response * aggregation(inputs)))
        // input nodes have: activ=ident, resp=0, agreg=none

        let mut map = BTreeMap::<_, f32>::new();

        for (node, input) in self.input.iter().zip(inputs.iter()) {
            for conn in node.forward_conns().iter().filter(|conn| conn.enabled()) {
                *map.entry(conn.output()).or_default() += (node.bias() * input) * conn.weight();
            }
        }

        // Currently iterates over all hidden nodes in the map in order of first
        // input > hidden > output, then if a == b, node with the least backward conns.
        //
        // How can we iterate in order least remaining input connections to visit?
        //
        // Mutating a counter that tracks the remaining input connections to visit
        // is a violation of the BTreeMap contract that states that "It is a logic
        // error for a key to be modified in such a way that the key's ordering
        // relative to any other key, as determined by the Ord trait, changes while
        // it is in the map."
        //
        // Removing an entry from the map using BTreeMap::pop_last, mutating the key,
        // then reinserting the entry is a viable solution that takes O(2log(n)) time.
        while let Some((Some(hidden), value)) = map.pop_last().map(|(node, value)| (node.hidden(), value)) {
            for conn in hidden.forward_conns().iter().filter(|conn| conn.enabled()) {
                *map.entry(conn.output()).or_default() += value * conn.weight();
            }
        }

        // TURN INTO AN ACCUMULATOR + AGGREGATOR
        todo!();
    }

    fn compat_dist(&self) -> f32 {
        todo!();
    }

    fn crossover(lhs: Self, rhs: Self) -> Self {
        todo!();
    }
}
