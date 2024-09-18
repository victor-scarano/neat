use crate::{node::{ConnInput, ConnOutput}, Population};
use std::{cmp::Ordering, hash};

#[derive(Clone)]
pub(crate) struct Conn<'g> {
    input: &'g dyn ConnInput<'g>,
    output: &'g dyn ConnOutput<'g>,
    weight: f32,
    enabled: bool,
    innov: usize,
}

impl<'g> Conn<'g> {
    pub(crate) fn new(input: &'g dyn ConnInput<'g>, output: &'g dyn ConnOutput<'g>) -> Self {
        Self {
            input,
            output,
            weight: f32::NAN,
            enabled: true,
            innov: Population::next_conn_innov(input, output)
        }
    }

    pub(crate) fn input(&self) -> &'g dyn ConnInput<'g> {
        self.input
    }

    pub(crate) fn output(&self) -> &'g dyn ConnOutput<'g> {
        self.output
    }

    pub(crate) fn weight(&self) -> f32 {
        self.weight
    }

    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn innov(&self) -> usize {
        self.innov
    }

    pub(crate) fn disable(&mut self) {
        self.enabled = false;
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
