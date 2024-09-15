#![feature(associated_type_defaults, cell_update)]
#![allow(dead_code)]

mod conn;
mod genome;
mod node;
mod population;

pub(crate) use conn::Conn;
pub use population::Population;
