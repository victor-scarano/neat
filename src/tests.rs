use crate::genome::Genome;
use std::{fs::{self, File}, io::Write};
use rand::{rngs::SmallRng, SeedableRng};

#[test]
fn figire_two() {
    let mut genome = Genome::<3, 1>::new();

    let mut rng = SmallRng::seed_from_u64(0);
    let conn = genome.mutate_add_conn(&mut rng).upgrade().unwrap();
    assert_eq!(conn.leading, *genome.inputs[0]);
    assert_eq!(conn.trailing, *genome.outputs[0]);

    let mut rng = SmallRng::seed_from_u64(1);
    let conn = genome.mutate_add_conn(&mut rng).upgrade().unwrap();
    assert_eq!(conn.leading, *genome.inputs[1]);
    assert_eq!(conn.trailing, *genome.outputs[0]);
    
    let mut rng = SmallRng::seed_from_u64(3);
    let conn = genome.mutate_add_conn(&mut rng).upgrade().unwrap();
    assert_eq!(conn.leading, *genome.inputs[1]);
    assert_eq!(conn.trailing, *genome.outputs[0]);

    let mut rng = SmallRng::seed_from_u64(1);
    genome.mutate_split_conn(&mut rng);
    // TODO: add asserts
    
    rng = SmallRng::seed_from_u64(0);
    genome.mutate_add_conn(&mut rng);
    // TODO: add asserts

    fs::create_dir_all("./tests/").unwrap();
    let mut file = File::create("./tests/figure2.dot").unwrap();
    write!(file, "{}", genome).unwrap();
}
