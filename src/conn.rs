use crate::{node::{ConnInput, ConnOutput}, Population};
use std::cmp::Ordering;

#[derive(Clone)]
pub(crate) struct Conn<'genome> {
    input: &'genome dyn ConnInput<'genome>,
    output: &'genome dyn ConnOutput<'genome>,
    weight: f32,
    enabled: bool,
    innov: usize,
}

impl<'genome> Conn<'genome> {
    pub(crate) fn new(input: &'genome dyn ConnInput<'genome>, output: &'genome dyn ConnOutput<'genome>) -> Self {
        Self {
            input,
            output,
            weight: f32::NAN,
            enabled: true,
            innov: Population::next_conn_innov(input, output)
        }
    }

    fn input(&self) -> &'genome dyn ConnInput {
        self.input
    }

    fn output(&self) -> &'genome dyn ConnOutput {
        self.output
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn innov(&self) -> usize {
        self.innov
    }
}

impl Eq for Conn<'_> {}

impl Ord for Conn<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.innov.cmp(&other.innov)
    }
}

impl PartialEq for Conn<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}

impl PartialOrd for Conn<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
