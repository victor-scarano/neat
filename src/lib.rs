#![feature(result_flattening, trait_upcasting)]
#![warn(clippy::all)]
#![allow(dead_code, clippy::from_over_into, clippy::mutable_key_type, unused_imports, unused_mut, unused_variables)]

pub(crate) mod activation;
mod config;
mod connection;
mod genome;
mod innovation;
pub(crate) mod node;
mod population;

pub use activation::Activation;
pub use config::Config;
pub(crate) use connection::Connection;
pub use genome::Genome;
pub(crate) use innovation::Innovation;
pub use population::Population;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

