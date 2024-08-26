use crate::{config::{Config, InitGenome}, Conn, Innov, node::{ConnInput, ConnOutput, Hidden, Input, Node, Output}};
use std::{any::Any, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, fmt, iter, sync::{Arc, OnceLock}};
use rand::{Rng, seq::{IteratorRandom, SliceRandom}};

#[derive(Clone)]
pub struct Genome {
    conns: BTreeSet<Arc<Conn>>,
    input_nodes: Box<[Arc<Input>]>,
    hidden_nodes: HashSet<Arc<Hidden>>,
    output_nodes: Box<[Arc<Output>]>,
    fitness: OnceLock<f32>,
    innov: Arc<Innov>,
    config: Arc<Config>,
}

impl Genome {
    pub(crate) fn new(rng: &mut impl Rng, innov: Arc<Innov>, config: Arc<Config>) -> Self {
        let mut genome = Self {
            conns: BTreeSet::new(),
            input_nodes: iter::repeat_with(|| {
                Arc::new(Input::new(rng, innov.clone(), config.clone()))
            }).take(config.num_inputs()).collect(),
            hidden_nodes: HashSet::new(),
            output_nodes: iter::repeat_with(|| {
                Arc::new(Output::new(rng, innov.clone(), config.clone()))
            }).take(config.num_outputs()).collect(),
            fitness: OnceLock::new(),
            innov: innov.clone(),
            config: config.clone(),
        };

        match config.init_genome() {
            InitGenome::Unconnected => (), // Make no further changes.
            InitGenome::FsNeatNoHidden => {
                let rand_input = genome.iter_input().choose(rng).unwrap();

                for output in genome.iter_output() {
                    genome.insert_conn(Conn::new(rand_input.clone(), output.clone(), genome.innov(), genome.config()));
                }
            },
            InitGenome::FsNeatHidden => todo!(),
            InitGenome::FullNoDirect => todo!(),
            InitGenome::FullDirect => todo!(),
            InitGenome::PartialNoDirect(prob) => todo!(),
            InitGenome::PartialDirect(prob) => todo!()
        }

        genome
    }

    pub(crate) fn iter_conns(&self) -> impl Iterator<Item = Arc<Conn>> {
        self.conns.iter().cloned().collect::<Vec<_>>().into_iter()
    }

    pub(crate) fn insert_conn(&mut self, conn: Conn) -> Arc<Conn> {
        let conn = Arc::new(conn);
        self.conns.insert(conn.clone());
        conn.clone()
    }

    #[allow(clippy::iter_cloned_collect)]
    pub(crate) fn iter_input(&self) -> impl Iterator<Item = Arc<Input>> {
        self.input_nodes.iter().cloned().collect::<Vec<_>>().into_iter()
    }

    pub(crate) fn iter_hidden(&self) -> impl Iterator<Item = Arc<Hidden>> {
        self.hidden_nodes.iter().cloned().collect::<Vec<_>>().into_iter()
    }

    pub(crate) fn insert_node(&mut self, node: Hidden) -> Arc<Hidden> {
        let node = Arc::new(node);
        self.hidden_nodes.insert(node.clone());
        self.hidden_nodes.get(&node).cloned().unwrap()
    }

    #[allow(clippy::iter_cloned_collect)]
    pub(crate) fn iter_output(&self) -> impl Iterator<Item = Arc<Output>> {
        self.output_nodes.iter().cloned().collect::<Vec<_>>().into_iter()
    }

    pub(crate) fn iter_conn_inputs(&self) -> impl Iterator<Item = Arc<dyn ConnInput>> {
        self.iter_input().map(|input| {
            input as Arc<dyn ConnInput>
        }).chain(self.iter_hidden().map(|hidden| {
            hidden as Arc<dyn ConnInput>
        }))
    }

    pub(crate) fn iter_conn_outputs(&self) -> impl Iterator<Item = Arc<dyn ConnOutput>> {
        self.iter_hidden().map(|hidden| {
            hidden as Arc<dyn ConnOutput>
        }).chain(self.iter_output().map(|output| {
            output as Arc<dyn ConnOutput>
        }))
    }

    pub(crate) fn fitness(&self) -> f32 {
        *self.fitness.get_or_init(f32::default)
    }

    pub(crate) fn set_fitness(&mut self, fitness: f32) {
        self.fitness.set(fitness).unwrap();
    }

    pub(crate) fn innov(&self) -> Arc<Innov> {
        self.innov.clone()
    }

    pub(crate) fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    pub(crate) fn mutate_add_conn(&mut self, rng: &mut impl Rng) {
        let rand_input = self.iter_conn_inputs().choose(rng).unwrap();
        let rand_output = self.iter_conn_outputs().filter(|output| {
            (output as &dyn Any).downcast_ref::<Hidden>().is_some_and(|lhs| {
                (&rand_input as &dyn Any).downcast_ref::<Hidden>().is_some_and(|rhs| lhs == rhs)
            })
        }).choose(rng).unwrap();

        let new_conn = self.insert_conn(Conn::new(rand_input.clone(), rand_output.clone(), self.innov(), self.config()));
        rand_input.insert_forward_conn(new_conn.clone());
        rand_output.insert_backward_conn(new_conn.clone());
    }

    pub(crate) fn mutate_split_conn(&mut self, rng: &mut impl Rng) {
        assert_ne!(self.conns.len(), 0);

        let rand_conn = self.iter_conns().filter(|conn| conn.enabled()).choose(rng).unwrap();

        rand_conn.disable();

        let new_node = self.insert_node(Hidden::new(rng, self.innov.clone(), self.config.clone()));

        let conn_a = self.insert_conn(Conn::new(rand_conn.input(), new_node.clone(), self.innov(), self.config()));
        let conn_b = self.insert_conn(Conn::new(new_node.clone(), rand_conn.output(), self.innov(), self.config()));

        rand_conn.input().insert_forward_conn(conn_a.clone());
        new_node.insert_backward_conn(conn_a.clone());
        new_node.insert_forward_conn(conn_b.clone());
        rand_conn.output().insert_backward_conn(conn_b.clone());
    }

    pub(crate) fn mutate_conn_weight(&mut self, rng: &mut impl Rng) {
        todo!();
    }

    pub(crate) fn activate(&self, inputs: impl AsRef<[f32]>) -> impl AsRef<[f32]> {
        assert_eq!(inputs.as_ref().len(), self.input_nodes.len());
        // activation ( bias + ( response * aggregation ( inputs ) ) )
        
        let mut map = BTreeMap::<Arc<dyn ConnOutput>, _>::new();

        for (node, input) in self.iter_input().zip(inputs.as_ref().iter()) {
            for conn in node.iter_forward_conns().filter(|conn| conn.enabled()) {
                *map.entry(conn.output()).or_default() += input * conn.weight();
            }
        }

        while let Some((Ok(node), value)) = map
            .pop_last()
            .map(|(node, value)| ((node as Arc<dyn Any + Send + Sync>).downcast::<Hidden>(), value))
        {
            // testing
            // node.activation(node.bias() + (node.response() * ))

            for conn in node.iter_forward_conns().filter(|conn| conn.enabled()) {
                *map.entry(conn.output()).or_default() += value * conn.weight();
            }
        }

        self.iter_output().map(|output| {
            map.get(&(output as Arc<dyn ConnOutput>)).cloned().unwrap()
        }).collect::<Vec<_>>()
    }

    pub(crate) fn compat_dist(lhs: &Self, rhs: &Self) -> f32 {
        todo!();
    }

    pub(crate) fn crossover(lhs: Self, rhs: Self, rng: &mut impl Rng) -> Self {
        todo!();
    }
}

impl fmt::Debug for Genome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = f.debug_struct("FeedForward");

        if !self.conns.is_empty() {
            output.field("conns", &self.iter_conns().collect::<Vec<_>>());
        }

        output.field("input", &self.iter_input().collect::<Vec<_>>());

        if !self.hidden_nodes.is_empty() {
            output.field("hidden", &self.iter_hidden().collect::<Vec<_>>());
        }

        output.field("output", &self.iter_output().collect::<Vec<_>>());

        if self.fitness.get().is_some() {
            output.field("fitness", &self.fitness());
        }

        output.finish()
    }
}

