use crate::genome::Genome;
use std::{fs::{self, File}, io::Write};
use rand::{rngs::SmallRng, SeedableRng};

#[test]
fn figire_two() {
    fs::create_dir_all("./tests/").unwrap();

    let mut genome = Genome::<3, 1>::new();

    let mut rng = SmallRng::seed_from_u64(0);
    let edge = genome.mutate_add_edge(&mut rng).upgrade().unwrap();
    assert_eq!(edge.tail, *genome.inputs[0]);
    assert_eq!(edge.head, *genome.outputs[0]);

    let mut rng = SmallRng::seed_from_u64(1);
    let edge = genome.mutate_add_edge(&mut rng).upgrade().unwrap();
    assert_eq!(edge.tail, *genome.inputs[1]);
    assert_eq!(edge.head, *genome.outputs[0]);
    
    let mut rng = SmallRng::seed_from_u64(3);
    let edge = genome.mutate_add_edge(&mut rng).upgrade().unwrap();
    assert_eq!(edge.tail, *genome.inputs[1]);
    assert_eq!(edge.head, *genome.outputs[0]);

    let mut rng = SmallRng::seed_from_u64(1);
    genome.mutate_split_edge(&mut rng);
    // TODO: add asserts
    
    rng = SmallRng::seed_from_u64(0);
    genome.mutate_add_edge(&mut rng);
    // TODO: add asserts

    let mut file = File::create("./tests/figure2.dot").unwrap();
    write!(file, "{}", genome).unwrap();
}

#[test]
fn figure_four() {
    fs::create_dir_all("./tests/figure4").unwrap();

    let mut parent = Genome::<3, 1>::new();

    let mut rng = SmallRng::seed_from_u64(0);
    let edge = parent.mutate_add_edge(&mut rng).upgrade().unwrap();
    assert_eq!(edge.tail, *parent.inputs[0]);
    assert_eq!(edge.head, *parent.outputs[0]);

    let mut rng = SmallRng::seed_from_u64(1);
    let edge = parent.mutate_add_edge(&mut rng).upgrade().unwrap();
    assert_eq!(edge.tail, *parent.inputs[1]);
    assert_eq!(edge.head, *parent.outputs[0]);
    
    let mut rng = SmallRng::seed_from_u64(3);
    let edge = parent.mutate_add_edge(&mut rng).upgrade().unwrap();
    assert_eq!(edge.tail, *parent.inputs[1]);
    assert_eq!(edge.head, *parent.outputs[0]);

    let mut rng = SmallRng::seed_from_u64(1);
    parent.mutate_split_edge(&mut rng);
    // TODO: add asserts
    
    let mut parent1 = parent.clone();
    let mut parent2 = parent.clone();

    // changes to parent for parent2
    let edge5 = parent2.edges
        .iter_ordered()
        .find(|edge| edge.tail.innov() == 4 && edge.head.innov() == 3)
        .cloned()
        .unwrap();
    
    edge5.enabled.set(false);

    let mut rng = SmallRng::seed_from_u64(2);
    parent2.mutate_split_edge(&mut rng);
    // TODO: add asserts
    
    // changes to parent for parent1
    let mut rng = SmallRng::seed_from_u64(0);
    let edge = parent1.mutate_add_edge(&mut rng).upgrade().unwrap();
    assert_eq!(edge.tail, *parent1.inputs[0]);
    assert_eq!(edge.head, **parent1.hiddens.iter().nth(0).unwrap());

    // more changes to parent for parent2 (edge 9 and 10)
    
    let mut file = File::create("./tests/figure4/parent.dot").unwrap();
    write!(file, "{}", parent).unwrap();
    
    let mut file = File::create("./tests/figure4/parent1.dot").unwrap();
    write!(file, "{}", parent1).unwrap();
    
    let mut file = File::create("./tests/figure4/parent2.dot").unwrap();
    write!(file, "{}", parent2).unwrap();
}

