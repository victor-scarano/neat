use core::mem::{self, MaybeUninit};

use crate::{genome::Genome, node::Hidden};
use rand::{rngs::SmallRng, SeedableRng};

#[test]
fn it_works() {
    let mut genome = Genome::<3, 1>::new();

    let mut rng = SmallRng::seed_from_u64(0);
    genome.mutate_add_edge(&mut rng);
    genome.mutate_split_edge(&mut rng);
    genome.mutate_split_edge(&mut rng);
    dbg!(genome.hiddens);
}
