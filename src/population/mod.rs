#![allow(dead_code, unused_imports)]

mod config;
mod innov;
mod species;

pub use config::Config;
pub(crate) use innov::Innov;
pub(crate) use species::Species;

pub struct Population;