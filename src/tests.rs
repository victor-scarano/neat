use crate::genome::Genome;
use rand::{rngs::SmallRng, SeedableRng};

#[test]
fn mutate_add_conn() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genome = Genome::<1, 1>::new();
    genome.mutate_add_conn(&mut rng);
}

#[test]
fn mutate_split_conn() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genome = Genome::<1, 1>::new();
    genome.mutate_add_conn(&mut rng);
    genome.mutate_split_conn(&mut rng);
}

#[test]
fn figire_two() {
    // uses seeded rng to recreate the neural network from figure two of stanley's paper.
    // note: does not model the recurrent connection

    let mut genome = Genome::<3, 1>::new();
    let mut rng;

    // add 1 -> 4
    rng = SmallRng::seed_from_u64(0);
    genome.mutate_add_conn(&mut rng);

    // add 2 -> 4
    rng = SmallRng::seed_from_u64(1);
    genome.mutate_add_conn(&mut rng);
    
    // add 3 -> 4
    rng = SmallRng::seed_from_u64(3);
    genome.mutate_add_conn(&mut rng);

    // split 2 -> 4 : 5
    rng = SmallRng::seed_from_u64(1);
    genome.mutate_split_conn(&mut rng);
    
    // add 1 -> 5
    rng = SmallRng::seed_from_u64(0);
    genome.mutate_add_conn(&mut rng);
}

#[test]
fn figure_four() {
    let mut rng;
    let mut parent = Genome::<3, 1>::new();

    // 1 -> 4
    rng = SmallRng::seed_from_u64(0);
    parent.mutate_add_conn(&mut rng);

    // 2 -> 4 (disabled)
    rng = SmallRng::seed_from_u64(1);
    parent.mutate_add_conn(&mut rng);
    parent.conns.iter_ordered().last().unwrap().enabled.set(false);

    // 3 -> 4
    rng = SmallRng::seed_from_u64(3);
    parent.mutate_add_conn(&mut rng);

    // split 2 -> 4 : 5
    rng = SmallRng::seed_from_u64(1);
    parent.mutate_split_conn(&mut rng);

    let mut parent2 = parent.clone();

    // disable 5 -> 4
    parent2.conns.iter_ordered().last().unwrap().enabled.set(false);

    // split 5 -> 4 : 6
    rng = SmallRng::seed_from_u64(0);
    parent2.mutate_split_conn(&mut rng);

    dbg!(parent2);
}

