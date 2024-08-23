use crate::{config::{Config, InitGenome}, Conn, Innov, node::{ConnInput, ConnOutput, Hidden, Input, Node, Output}};
use std::{any::Any, cell::{OnceCell, RefCell}, collections::{BTreeMap, BTreeSet, HashMap, HashSet}, fmt, iter, sync::Arc};
use rand::{seq::{IteratorRandom, SliceRandom}, Rng};

#[derive(Clone)]
pub struct Genome {
    conns: BTreeSet<Arc<Conn>>,
    input: Box<[Arc<Input>]>,
    hidden: HashSet<Arc<Hidden>>,
    output: Box<[Arc<Output>]>,
    fitness: OnceCell<f32>,
}

impl Genome {
    pub(crate) fn insert_conn(&mut self, conn: Conn) -> Arc<Conn> {
        let conn = Arc::new(conn);
        self.conns.insert(conn.clone());
        conn.clone()
    }

    pub(crate) fn insert_node(&mut self, node: Hidden) -> Arc<Hidden> {
        let node = Arc::new(node);
        self.hidden.insert(node.clone());
        self.hidden.get(&node).cloned().unwrap()
    }

    pub(crate) fn add_conn(
        &mut self,
        input: Arc<dyn ConnInput>,
        output: Arc<dyn ConnOutput>,
        innov: &Innov,
        config: &Config,
    ) -> Arc<Conn> {
        let new_conn = self.insert_conn(Conn::new(input.clone(), output.clone(), innov, config));

        input.insert_forward_conn(new_conn.clone());
        output.insert_backward_conn(new_conn.clone());

        new_conn
    }

    pub(crate) fn split_conn(
        &mut self,
        old_conn: Arc<Conn>,
        rng: &mut impl Rng,
        innov: &Innov,
        config: &Config,
    ) -> (Arc<Conn>, Arc<Conn>) {
        old_conn.disable();

        let new_node = self.insert_node(Hidden::new(rng, innov, config));

        let conn_a = self.insert_conn(Conn::new(old_conn.input(), new_node.clone(), innov, config));
        let conn_b = self.insert_conn(Conn::new(new_node.clone(), old_conn.output(), innov, config));

        old_conn.input().insert_forward_conn(conn_a.clone());
        new_node.insert_backward_conn(conn_a.clone());
        new_node.insert_forward_conn(conn_b.clone());
        old_conn.output().insert_backward_conn(conn_b.clone());

        (conn_a, conn_b)
    }

    pub(crate) fn fitness(&self) -> f32 {
        let fitness = self.fitness.get();
        fitness.cloned().unwrap()
    }

    pub(crate) fn iter_conns(&self) -> impl Iterator<Item = Arc<Conn>> {
        self.conns.iter().cloned().collect::<Vec<_>>().into_iter()
    }

    pub(crate) fn iter_input(&self) -> impl Iterator<Item = Arc<Input>> {
        #[allow(clippy::iter_cloned_collect)]
        self.input.iter().cloned().collect::<Vec<_>>().into_iter()
    }

    pub(crate) fn iter_hidden(&self) -> impl Iterator<Item = Arc<Hidden>> {
        self.hidden.iter().cloned().collect::<Vec<_>>().into_iter()
    }

    pub(crate) fn iter_output(&self) -> impl Iterator<Item = Arc<Output>> {
        #[allow(clippy::iter_cloned_collect)]
        self.output.iter().cloned().collect::<Vec<_>>().into_iter()
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

    pub(crate) fn new(rng: &mut impl Rng, innov: &Innov, config: &Config) -> Self {
        let mut genome = Self {
            conns: BTreeSet::new(),
            input: (0..config.num_inputs()).map(|_| Arc::new(Input::new(rng, innov, config))).collect(),
            hidden: HashSet::new(),
            output: (0..config.num_outputs()).map(|_| Arc::new(Output::new(rng, innov, config))).collect(),
            fitness: OnceCell::new(),
        };

        match config.init_genome() {
            InitGenome::Unconnected => (), // Make no further changes.
            InitGenome::FsNeatNoHidden => {
                let rand_input = genome.iter_input().choose(rng).unwrap();

                for output in genome.iter_output() {
                    genome.insert_conn(Conn::new(rand_input.clone(), output.clone(), innov, config));
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

    pub(crate) fn mutate_add_conn(&mut self, rng: &mut impl Rng, innov: &Innov, config: &Config) {
        let mut conn_inputs = self.iter_conn_inputs().collect::<Vec<_>>();
        conn_inputs.shuffle(rng);

        let mut conn_outputs = self.iter_conn_outputs().collect::<Vec<_>>();
        conn_outputs.shuffle(rng);

        let rand_input = conn_inputs.into_iter().find(|node| {
            // (possible forward conns) - (node's forward conns) > 0 node has at least one valid output node.
            conn_outputs.len()
                .saturating_sub((node.clone() as Arc<dyn Any>).downcast_ref::<Hidden>().is_some_and(|downcasted| {
                    self.hidden.contains(downcasted)
                }) as usize)
                .saturating_sub(node.num_forward_conns()) > 0
        }).unwrap();

        let random_output = conn_outputs.into_iter().find(|node| {
            !node.contains_backward_conn_by(&mut |conn| conn.input() == rand_input.clone())
        }).unwrap();

        self.add_conn(rand_input, random_output, innov, config);
    }

    pub(crate) fn mutate_split_conn(&mut self, rng: &mut impl Rng, innov: &Innov, config: &Config) {
        assert_ne!(self.conns.len(), 0);
        let rand_conn = self.iter_conns().filter(|conn| conn.enabled()).choose(rng).unwrap();
        self.split_conn(rand_conn, rng, innov, config);
    }

    pub(crate) fn mutate_conn_weight(&mut self, rng: &mut impl Rng, config: &Config) {
        todo!();
    }

    pub(crate) fn activate(&self, inputs: impl AsRef<[f32]>, config: &Config) -> impl AsRef<[f32]> {
        assert_eq!(inputs.as_ref().len(), self.input.len());
        // activation ( bias + ( response * aggregation ( inputs ) ) )
        
        let mut map: BTreeMap<Arc<dyn ConnOutput>, f32> = BTreeMap::new();

        for (node, input) in self.iter_input().zip(inputs.as_ref().iter()) {
            for conn in node.iter_enabled_forward_conns() {
                *map.entry(conn.output()).or_default() += input * conn.weight();
            }
        }

        while let Some((Ok(node), value)) = map
            .pop_last()
            .and_then(|(node, value)| Some(((node as Arc<dyn Any + Send + Sync>).downcast::<Hidden>(), value)))
        {
            for conn in node.iter_enabled_forward_conns() {
                *map.entry(conn.output()).or_default() += value * conn.weight();
            }
        }

        self.iter_output().map(|output| map.get(&(output as Arc<dyn ConnOutput>)).cloned().unwrap()).collect::<Vec<_>>()
    }

    pub(crate) fn set_fitness(&mut self, fitness: f32, config: &Config) {
        self.fitness.set(fitness).unwrap();
    }

    pub(crate) fn compat_dist(lhs: &Self, rhs: &Self, config: &Config) -> f32 {
        todo!();
    }

    pub(crate) fn crossover(lhs: Self, rhs: Self, rng: &mut impl Rng, config: &Config) -> Self {
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

        if !self.hidden.is_empty() {
            output.field("hidden", &self.iter_hidden().collect::<Vec<_>>());
        }

        output.field("output", &self.iter_output().collect::<Vec<_>>());

        if self.fitness.get().is_some() {
            output.field("fitness", &self.fitness());
        }

        output.finish()
    }
}

