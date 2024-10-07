#![feature(cell_update, debug_closure_helpers, hash_set_entry)]
#![warn(clippy::missing_const_for_fn, missing_copy_implementations, missing_debug_implementations)]
#![allow(dead_code, clippy::mutable_key_type, unused_variables)]

mod conn;
mod genome;
mod node;
mod population;

#[cfg(test)]
mod tests {
    use crate::genome::Genome;
    use rand::rngs::mock::StepRng;

    #[test]
    fn it_works() {
        for step in 1..=213 {
            println!("Running step {}...", step);
            let mut rng = StepRng::new(0, step);
            let mut genome = Genome::<1, 1>::new();
            genome.mutate_add_conn(&mut rng);
            genome.mutate_split_conn(&mut rng);
            println!("Finished running step {}", step);
        }
    }
}
