use crate::genome::Genome;
use rand::{rngs::SmallRng, SeedableRng};

// #[test]
fn mutate_add_conn() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genome = Genome::<1, 1>::new();
    genome.mutate_add_conn(&mut rng);
}

// #[test]
fn mutate_split_conn() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genome = Genome::<1, 1>::new();
    genome.mutate_add_conn(&mut rng);
    genome.mutate_split_conn(&mut rng);
}

#[test]
fn activate() {
    // uses seeded rng to recreate the neural network from stanley's paper.
    // note: does not model the recurrent connection

    let mut genome = Genome::<3, 1>::new();

    // add 1 -> 4
    let mut rng = SmallRng::seed_from_u64(0);
    genome.mutate_add_conn(&mut rng);

    // add 2 -> 4
    let mut rng = SmallRng::seed_from_u64(1);
    genome.mutate_add_conn(&mut rng);
    
    // add 3 -> 4
    let mut rng = SmallRng::seed_from_u64(3);
    genome.mutate_add_conn(&mut rng);

    // split 2 -> 4 : 5
    let mut rng = SmallRng::seed_from_u64(1);
    genome.mutate_split_conn(&mut rng);
    
    // add 1 -> 5
    let mut rng = SmallRng::seed_from_u64(0);
    genome.mutate_add_conn(&mut rng);
    
    dbg!(genome);
}

// #[test]
fn crossover() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut a = Genome::<2, 1>::new();
    a.mutate_add_conn(&mut rng);
    a.mutate_split_conn(&mut rng);

    let mut rng = SmallRng::seed_from_u64(1);
    let mut b = Genome::<2, 1>::new();
    b.mutate_add_conn(&mut rng);
    b.mutate_split_conn(&mut rng);

    println!("{}", "A".repeat(99));
    for conn in a.conns.iter_ordered() {
        dbg!(conn);
    }
    println!();

    println!("{}", "B".repeat(99));
    for conn in b.conns.iter_ordered() {
        dbg!(conn);
    }
    println!("\n{}", "CROSSOVER ".repeat(10));

    let mut rng = SmallRng::seed_from_u64(2);
    let child = Genome::crossover(a, b, &mut rng);
}
