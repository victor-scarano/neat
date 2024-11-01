// #![cfg_attr(not(test), no_std)]
#![feature(anonymous_lifetime_in_impl_trait, cell_update, debug_closure_helpers, thread_local)]
#![allow(dead_code, clippy::mutable_key_type, unused_variables)]

mod conn;
mod genome;
mod node;
mod pop;

#[cfg(test)]
mod tests {
    use crate::genome::Genome;
    use rand::rngs::mock::StepRng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn mutate_add_conn() {
        let mut rng = StepRng::new(0, 1);
        let mut genome = Genome::<1, 1>::new();
        genome.mutate_add_conn(&mut rng);
    }

    #[test]
    fn mutate_split_conn() {
        let mut rng = StepRng::new(0, 1);
        let mut genome = Genome::<1, 1>::new();
        genome.mutate_add_conn(&mut rng);
        genome.mutate_split_conn(&mut rng);
    }

    #[test]
    fn activate() {
        // uses seeded rngs to recreate the neural network from stanley's paper.
        // with no connection weights, identity activation, 0.0 bias, and 1.0
        // response, and average aggregator for all nodes, the genome's activation
        // should be 11/6. IT PASSES TESTS!!!! :))

        let mut genome = Genome::<3, 1>::new();

        let mut rng = StdRng::seed_from_u64(0);
        genome.mutate_add_conn(&mut rng); // add 1 -> 4

        let mut rng = StdRng::seed_from_u64(2);
        genome.mutate_add_conn(&mut rng); // add 2 -> 4

        let mut rng = StdRng::seed_from_u64(1);
        genome.mutate_add_conn(&mut rng); // add 3 -> 4

        let mut rng = StdRng::seed_from_u64(2);
        genome.mutate_split_conn(&mut rng); // split 2 -> 4 : 5

        let mut rng = StdRng::seed_from_u64(4);
        genome.mutate_add_conn(&mut rng); // add 1 -> 5

        let activation = genome.activate([1.0, 2.0, 3.0]);
        assert_eq!(activation[0], 11.0 / 6.0);
    }
}
