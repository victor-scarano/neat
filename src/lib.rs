#![feature(cell_update, debug_closure_helpers)]
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
        let rng = StepRng::new(0, 1);
        let mut genome = Genome::<1, 3, _>::new(rng);
        dbg!(&genome);
        genome.mutate_add_conn();
        dbg!(&genome);
    }
}
