use crate::genome::Genome;
use std::{fs::{self, File}, io::Write};
use rand::{rngs::SmallRng, SeedableRng};

#[test]
fn figire_two() {
    fs::create_dir_all("./tests/").unwrap();

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

    let mut file = File::create("./tests/figure2.dot").unwrap();
    write!(file, "{}", genome).unwrap();
}

#[test]
fn figure_four() {
    fs::create_dir_all("./tests/figure4").unwrap();

    let mut parent = Genome::<3, 1>::new();

    let mut rng = SmallRng::seed_from_u64(0);
    let conn = parent.mutate_add_conn(&mut rng).upgrade().unwrap();
    assert_eq!(conn.leading, *parent.inputs[0]);
    assert_eq!(conn.trailing, *parent.outputs[0]);

    let mut rng = SmallRng::seed_from_u64(1);
    let conn = parent.mutate_add_conn(&mut rng).upgrade().unwrap();
    assert_eq!(conn.leading, *parent.inputs[1]);
    assert_eq!(conn.trailing, *parent.outputs[0]);
    
    let mut rng = SmallRng::seed_from_u64(3);
    let conn = parent.mutate_add_conn(&mut rng).upgrade().unwrap();
    assert_eq!(conn.leading, *parent.inputs[1]);
    assert_eq!(conn.trailing, *parent.outputs[0]);

    let mut rng = SmallRng::seed_from_u64(1);
    parent.mutate_split_conn(&mut rng);
    // TODO: add asserts
    
    let mut parent1 = parent.clone();
    let mut parent2 = parent.clone();

    // changes to parent for parent2
    let conn5 = parent2.conns
        .iter_ordered()
        .find(|conn| conn.leading.innov() == 4 && conn.trailing.innov() == 3)
        .cloned()
        .unwrap();
    
    conn5.enabled.set(false);

    let mut rng = SmallRng::seed_from_u64(2);
    parent2.mutate_split_conn(&mut rng);
    // TODO: add asserts
    
    // changes to parent for parent1
    let mut rng = SmallRng::seed_from_u64(0);
    let conn = parent1.mutate_add_conn(&mut rng).upgrade().unwrap();
    assert_eq!(conn.leading, *parent1.inputs[0]);
    assert_eq!(conn.trailing, **parent1.hiddens.iter().nth(0).unwrap());

    // more changes to parent for parent2 (conn 9 and 10)
    
    let mut file = File::create("./tests/figure4/parent.dot").unwrap();
    write!(file, "{}", parent).unwrap();
    
    let mut file = File::create("./tests/figure4/parent1.dot").unwrap();
    write!(file, "{}", parent1).unwrap();
    
    let mut file = File::create("./tests/figure4/parent2.dot").unwrap();
    write!(file, "{}", parent2).unwrap();
}

