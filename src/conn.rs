use crate::{node::{ConnInput, ConnOutput}, Population};
use std::{cell::Cell, cmp::Ordering, hash};

#[derive(Clone)]
pub(crate) struct Conn<'genome> {
    input: ConnInput<'genome>,
    output: ConnOutput<'genome>,
    weight: f32,
    enabled: Cell<bool>,
    innov: usize,
}

impl<'genome> Conn<'genome> {
    pub(crate) fn new(input: ConnInput<'genome>, output: ConnOutput<'genome>) -> Self {
        // TODO: Assert that input and output are not pointing to the same connection.
        Self {
            input: input.clone(),
            output: output.clone(),
            weight: f32::NAN,
            enabled: Cell::new(true),
            innov: Population::next_conn_innov(input, output)
        }
    }

    pub(crate) fn input(&self) -> ConnInput<'genome> {
        self.input.clone()
    }

    pub(crate) fn output(&self) -> ConnOutput<'genome> {
        self.output.clone()
    }

    pub(crate) fn weight(&self) -> f32 {
        self.weight
    }

    pub(crate) fn enabled(&self) -> bool {
        self.enabled.get()
    }

    pub(crate) fn innov(&self) -> usize {
        self.innov
    }

    pub(crate) fn disable(&self) {
        self.enabled.set(false);
    }
}

impl<'genome> Eq for Conn<'genome> {}

impl<'genome> hash::Hash for Conn<'genome> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl<'genome> Ord for Conn<'genome> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.innov.cmp(&other.innov)
    }
}

impl<'genome> PartialEq for Conn<'genome> {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}

impl<'genome> PartialOrd for Conn<'genome> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
