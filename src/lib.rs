#![allow(dead_code, clippy::mutable_key_type, unused_variables)]
#![feature(cell_update)]

mod conn;
mod genome;
mod node;
mod population;

pub(crate) use conn::Conn;
pub use population::Population;
