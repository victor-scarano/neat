use crate::genome::Genome;
use rand::{rngs::SmallRng, SeedableRng};

fn mutate_split_edge<const I: usize, const O: usize>(
    genome: &mut Genome<I, O>,
    assert: impl Fn(&Genome<I, O>) -> bool,
) {
    for seed in 0..u8::MAX {
        let mut clone = genome.clone();
        let mut rng = SmallRng::seed_from_u64(seed as u64);
        clone.mutate_split_edge(&mut rng);
        if assert(&clone) {
            let mut rng = SmallRng::seed_from_u64(seed as u64);
            genome.mutate_split_edge(&mut rng);
            return;
        }
    };
    panic!("could not find seed that validates the assertion after {} attempts", u8::MAX);
}

#[test]
fn it_works() {
    let mut genome = Genome::<1, 1>::new();

    let mut rng = SmallRng::seed_from_u64(0);
    genome.mutate_add_edge(&mut rng);

    mutate_split_edge(&mut genome, |_| true);

    dbg!(&genome.hiddens);
}

