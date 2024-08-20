#![warn(clippy::all)]
#![allow(dead_code, clippy::from_over_into, clippy::mutable_key_type, unused_variables)]

pub(crate) mod activation;
mod config;
mod connection;
mod genome;
mod innovation;
mod node;
mod population;

pub use activation::Activation;
pub use config::Config;
pub(crate) use connection::Connection;
pub use genome::Genome;
pub(crate) use innovation::Innovation;
pub(crate) use node::Node;
pub use population::Population;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

