// #![cfg_attr(not(test), no_std)]
#![feature(
    allocator_api, anonymous_lifetime_in_impl_trait,
    arbitrary_self_types_pointers, cell_update, iter_collect_into,
    debug_closure_helpers, maybe_uninit_slice, non_null_from_ref, thread_local
)]
#![allow(dead_code, clippy::mutable_key_type, unused_variables, unused_imports)]

mod edge;
mod genome;
mod node;
mod pop;
#[cfg(test)]
mod tests;
