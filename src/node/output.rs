extern crate alloc;
use crate::{pop::Pop, node::*, node::Accum};
use core::{array, cell::Cell, cmp, fmt, hash, slice};
use alloc::rc::Rc;
use hashbrown::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    layer: Cell<usize>,
    activation: Cell<fn(f32) -> f32>,
    aggregator: fn(&[f32]) -> f32,
    response: f32,
    bias: f32,
    innov: usize,
}

impl Output {
    pub fn new_in<const I: usize>(innov: usize, bump: Bump) -> Rc<Self, Bump> {
        Pop::next_node_innov();
        Rc::new_in(
            Self {
                layer: 1.into(),
                activation: Cell::new(|x| x),
                aggregator: |values| values.iter().sum::<f32>() / (values.len() as f32),
                response: 1.0,
                bias: 0.0,
                innov: I - innov,
            },
            bump
        )
    }

    pub fn index<const I: usize>(&self) -> usize {
        self.innov - I
    }

    pub fn eval(self: &Rc<Self, Bump>, map: &mut HashMap<Head, Accum>) -> f32 {
        let input = map.get_mut(&Head::from(self.clone())).unwrap().eval(self.aggregator);
        self.activate(self.bias() + (self.response() * input))
    }

    pub fn clone_in(&self, bump: Bump) -> Rc<Self, Bump> {
        Rc::new_in(self.clone(), bump)
    }
}

impl Node for Output {
    fn layer(&self) -> usize { self.layer.get() }
    fn bias(&self) -> f32 { self.bias }
    fn innov(&self) -> usize { self.innov }
    fn update_layer(&self, layer: usize) { self.layer.update(|current| cmp::max(current, layer)); }
    fn activate(&self, x: f32) -> f32 { self.activation.get()(x)}
    fn response(&self) -> f32 { self.response }
    fn aggregator(&self) -> fn(&[f32]) -> f32 { self.aggregator }
}

impl Eq for Output {}

impl hash::Hash for Output {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.layer.get().hash(state);
        self.response.to_bits().hash(state);
        self.bias.to_bits().hash(state);
        self.innov.hash(state);
    }
}

pub struct Outputs<const O: usize>(Box<[Rc<Output, Bump>; O]>);

impl<const O: usize> Outputs<O> {
    pub fn new_in<const I: usize>(bump: Bump) -> Self {
        Self(Box::new(array::from_fn::<_, O, _>(|innov| Output::new_in::<I>(innov, bump.clone()))))
    }

    pub fn get(&self, index: usize) -> Option<Rc<Output, Bump>> {
        self.0.get(index).cloned()
    }

    pub fn eval_nth(&self, n: usize, map: &mut HashMap<Head, Accum>) -> f32 {
        self.get(n).unwrap().eval(map)
    }

    pub fn iter(&self) -> slice::Iter<Rc<Output, Bump>> {
        self.0.iter()
    }
}

impl<const O: usize> fmt::Debug for Outputs<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(&mut f.debug_map(), |f, ref output| {
            f.key_with(|f| fmt::Pointer::fmt(output, f)).value(output)
        }).finish()
    }
}

impl<const O: usize> TryFrom<Vec<Rc<Output, Bump>>> for Outputs<O> {
    type Error = <Box<[Rc<Output, Bump>; O]> as TryFrom<Vec<Rc<Output, Bump>>>>::Error;

    fn try_from(value: Vec<Rc<Output, Bump>>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

