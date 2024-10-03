use crate::{node::*, population::Population};
use std::{cell::Cell, cmp::Ordering, hash};

#[derive(Clone, Debug)]
pub struct Conn<'genome> {
    input: ConnInput<'genome>,
    output: ConnOutput<'genome>,
    level: usize,
    weight: f32,
    enabled: Cell<bool>,
    innov: usize,
}

impl<'genome> Conn<'genome> {
    pub fn new(input: ConnInput<'genome>, output: ConnOutput<'genome>) -> Self {
        assert_ne!(input, output);

        output.update_level(input.level() + 1);

        Self {
            input: input.clone(),
            output: output.clone(),
            level: input.level(),
            weight: f32::NAN,
            enabled: true.into(),
            innov: Population::next_conn_innov(input, output)
        }
    }

    pub fn conn_input(&self) -> ConnInput<'genome> {
        self.input.clone()
    }

    pub fn conn_output(&self) -> ConnOutput<'genome> {
        self.output.clone()
    }

    pub const fn level(&self) -> usize {
        self.level
    }

    pub const fn weight(&self) -> f32 {
        self.weight
    }

    pub fn enabled(&self) -> bool {
        self.enabled.get()
    }

    pub const fn innov(&self) -> usize {
        self.innov
    }

    pub fn disable(&self) {
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
        self.level.cmp(&other.level)
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

