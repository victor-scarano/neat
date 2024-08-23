#![feature(result_flattening, trait_upcasting)]
#![warn(clippy::all)]
#![allow(dead_code, clippy::from_over_into, clippy::mutable_key_type, unused_imports, unused_mut, unused_variables)]

pub(crate) mod activation;
mod config;
mod conn;
pub(crate) mod genome;
mod innov;
pub(crate) mod node;
mod population;

pub use activation::Activation;
pub use config::Config;
pub(crate) use conn::Conn;
pub(crate) use genome::Genome;
pub(crate) use innov::Innov;
pub use population::Population;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::num::NonZero;

    /// Checks if the ordering of the inserted connection in the [`BTreeSet`] is correct. Does not check the validity
    /// of connection innovations in complex genomes.
    #[test]
    fn insert_conn() {
        let mut rng = rand::thread_rng();
        let innov = Innov::default();
        let config = Config::new(NonZero::new(1).unwrap(), NonZero::new(3).unwrap(), NonZero::new(1).unwrap());
        let mut genome = Genome::new(&mut rng, &innov, &config);

        let conns = (0..3).map(|n| {
            let input = genome.iter_input().nth(n).unwrap();
            let output = genome.iter_output().nth(0).unwrap();

            genome.insert_conn(Conn::new(
                input.clone(),
                output.clone(),
                f32::MAX,
                innov.new_conn_innovation(input.clone(), output.clone()),
            ))
        }).collect::<Vec<_>>();

        assert_eq!(conns, genome.iter_conns().collect::<Vec<_>>())
    }

    /// Checks 
    #[test]
    fn insert_node() {
    }
}

