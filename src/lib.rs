#![cfg_attr(not(test), no_std)]
#![feature(cell_update, iter_collect_into, debug_closure_helpers, maybe_uninit_slice, thread_local)]
#![allow(dead_code, unused_variables)]
// #![warn(clippy::cargo, clippy::style)]

mod edge;
mod fitness;
mod genome;
mod node;
mod pop;
mod tests;
