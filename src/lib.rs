#![cfg_attr(not(test), no_std)]
#![feature(box_vec_non_null, cell_update, iter_collect_into, debug_closure_helpers, maybe_uninit_uninit_array, maybe_uninit_slice, thread_local)]
#![allow(dead_code, unused_variables)]
// #![warn(clippy::cargo, clippy::style)]

mod arena;
mod edge;
mod fitness;
mod genome;
mod node;
mod pop;
mod tests;
