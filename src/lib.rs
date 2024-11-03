#![no_std]
#![feature(anonymous_lifetime_in_impl_trait, cell_update, debug_closure_helpers, thread_local)]
#![allow(dead_code, clippy::mutable_key_type, unused_variables)]

mod conn;
mod genome;
mod node;
mod pop;
#[cfg(test)]
mod tests;
