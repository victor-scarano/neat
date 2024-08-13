use crate::node::Node;
use std::{cell::Cell, cmp::Ordering, fmt, hash::{Hash, Hasher}, rc::Rc};

/// A connection between two nodes or neurons within the genome, coupled with other relevant data.
#[derive(Default)]
pub(crate) struct Conn {
    input: Cell<Rc<Node>>,
    output: Cell<Rc<Node>>,
    weight: f32,
    enabled: Cell<bool>,
    innov: u32,
}

impl Conn {
    /// Constructs a new connection.
    pub fn new(input: Rc<Node>, output: Rc<Node>, weight: f32, innov: u32) -> Self {
        Self {
            input: Cell::new(input),
            output: Cell::new(output),
            weight,
            enabled: Cell::new(true),
            innov,
        }
    }

    /// Returns an [`Rc<Node>`] of the node feeding into the connection.
    pub fn input(&self) -> Rc<Node> {
        let ret = self.input.take();
        self.input.set(ret.clone());
        ret.clone()
    }

    /// Returns an [`Rc<Node>`] of the node feeding out of the connection.
    pub fn output(&self) -> Rc<Node> {
        let ret = self.output.take();
        self.output.set(ret.clone());
        ret.clone()
    }

    pub fn set_input(&self, f: impl Fn(Rc<Node>) -> Rc<Node>) {
        self.input.set(f(self.input()));
    }

    pub fn set_output(&self, f: impl Fn(Rc<Node>) -> Rc<Node>) {
        self.output.set(f(self.output()));
    }

    /// Returns the weight of the connection.
    pub fn weight(&self) -> f32 {
        self.weight
    }

    /// Returns the enabled status of the connection.
    pub fn enabled(&self) -> bool {
        self.enabled.get()
    }

    /// Disables the connection.
    pub fn disable(&self) {
        self.enabled.set(false);
    }

    /// Returns an iterator over the connections input and output nodes.
    pub fn nodes(&self) -> impl Iterator<Item = Rc<Node>> {
        [self.input(), self.output()].into_iter()
    }
}

impl Clone for Conn {
    fn clone(&self) -> Self {
        Self {
            input: Cell::new(self.input()),
            output: Cell::new(self.output()),
            weight: self.weight,
            enabled: self.enabled.clone(),
            innov: self.innov,
        }
    }
}

impl Eq for Conn {}

impl fmt::Debug for Conn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Conn")
            .field("addr", &format_args!("{:?}", self as *const Self))
            .field("input", &format_args!("{:p}", self.input()))
            .field("output", &format_args!("{:p}", self.output()))
            .field("weight", &self.weight)
            .field("enabled", &self.enabled())
            .field("innov", &self.innov)
            .finish()
    }
}

impl Hash for Conn {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.input()).hash(state);
        Rc::as_ptr(&self.output()).hash(state);
    }
}

impl Ord for Conn {
    fn cmp(&self, other: &Self) -> Ordering {
        self.innov.cmp(&other.innov)
    }
}

impl PartialEq for Conn {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.input(), &other.input()) && Rc::ptr_eq(&self.output(), &other.output())
    }
}

impl PartialOrd for Conn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
