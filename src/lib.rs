// #![cfg_attr(not(test), no_std)]
#![no_std]
#![feature(cell_update, debug_closure_helpers, thread_local)]
#![allow(dead_code, clippy::mutable_key_type, unused_variables)]

mod conn;
mod genome;
mod node;
mod pop;

#[cfg(test)]
mod tests {
    use crate::genome::Genome;
    use rand::rngs::mock::StepRng;

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
}
