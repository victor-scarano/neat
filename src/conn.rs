use crate::{node::{ConnInput, ConnOutput}, Population};
use std::{cell::Cell, cmp::Ordering, hash};

#[derive(Clone)]
pub(crate) struct Conn<'g> {
    input: ConnInput<'g>,
    output: ConnOutput<'g>,
    weight: f32,
    enabled: Cell<bool>,
    innov: usize,
}

impl<'g> Conn<'g> {
    pub(crate) fn new(input: ConnInput<'g>, output: ConnOutput<'g>) -> Self {
        // TODO: Assert that input and output are not pointing to the same connection.
        Self {
            input: input.clone(),
            output: output.clone(),
            weight: f32::NAN,
            enabled: Cell::new(true),
            innov: Population::next_conn_innov(input, output)
        }
    }

    pub(crate) fn input(&self) -> ConnInput<'g> {
        self.input.clone()
    }

    pub(crate) fn output(&self) -> ConnOutput<'g> {
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

impl<'g> Eq for Conn<'g> {}

impl<'g> hash::Hash for Conn<'g> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl<'g> Ord for Conn<'g> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.innov.cmp(&other.innov)
    }
}

impl<'g> PartialEq for Conn<'g> {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}

impl<'g> PartialOrd for Conn<'g> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
