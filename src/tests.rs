use core::mem::{self, MaybeUninit};

use crate::{genome::Genome, node::Hidden};
use rand::{rngs::SmallRng, SeedableRng};

#[test]
fn it_works() {
    let mut genome = Genome::<3, 1>::new();

    genome.mutate_add_edge(0, 0);
    genome.mutate_add_edge(1, 0);
    genome.mutate_add_edge(2, 0);

    // let mut rng = SmallRng::seed_from_u64(0);
    // genome.mutate_split_edge(&mut rng);

    for (idx, hidden) in genome.hiddens.iter().enumerate() {
        dbg!(idx, hidden);
    }
}
