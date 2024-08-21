use crate::{Activation, Config, Connection, Innovation};
use std::{cell::RefCell, cmp::Ordering, collections::BTreeSet, fmt, hash, rc::Rc};
use rand::{seq::IteratorRandom, Rng};

#[derive(Default, Clone)]
pub(crate) struct Node {
    kind: NodeKind,
    forward: RefCell<BTreeSet<Rc<Connection>>>,
    backward: RefCell<BTreeSet<Rc<Connection>>>,
    activation: RefCell<Activation>,
    // bias: f32,
    innovation: u32,
}

impl Node {
    pub(crate) fn new_input(innov: &Innovation, config: &Config) -> Self {
        Self {
            kind: NodeKind::Input,
            forward: RefCell::new(BTreeSet::new()),
            backward: RefCell::new(BTreeSet::new()),
            activation: RefCell::new(config.default_activation()),
            innovation: innov.new_node(),
        }
    }

    pub(crate) fn new_hidden(innov: &Innovation, config: &Config) -> Self {
        Self {
            kind: NodeKind::Hidden,
            forward: RefCell::new(BTreeSet::new()),
            backward: RefCell::new(BTreeSet::new()),
            activation: RefCell::new(config.default_activation()),
            innovation: innov.new_node(),
        }
    }

    pub(crate) fn new_output(innov: &Innovation, config: &Config) -> Self {
        Self {
            kind: NodeKind::Output,
            forward: RefCell::new(BTreeSet::new()),
            backward: RefCell::new(BTreeSet::new()),
            activation: RefCell::new(config.default_activation()),
            innovation: innov.new_node(),
        }
    }

    pub(crate) fn innovation(&self) -> u32 {
        self.innovation
    }

    pub(crate) fn is_input(&self) -> bool {
        self.kind == NodeKind::Input
    }

    pub(crate) fn is_hidden(&self) -> bool {
        self.kind == NodeKind::Hidden
    }

    pub(crate) fn is_output(&self) -> bool {
        self.kind == NodeKind::Output
    }

    pub(crate) fn insert_forward_conn(&self, conn: Rc<Connection>) {
        if self.kind != NodeKind::Output {
            self.forward.borrow_mut().insert(conn);
        }
    }

    pub(crate) fn insert_backward_conn(&self, conn: Rc<Connection>) {
        if self.kind != NodeKind::Input {
            self.backward.borrow_mut().insert(conn);
        }
    }

    pub(crate) fn num_forward_conns(&self) -> usize {
        self.forward.borrow().len()
    }

    pub(crate) fn num_backward_conns(&self) -> usize {
        self.backward.borrow().len()
    }

    pub(crate) fn iter_enabled_forward_conns(&self) -> impl Iterator<Item = Rc<Connection>> + '_ {
        self.forward.borrow().iter().filter(|conn| conn.enabled()).cloned().collect::<Vec<_>>().into_iter()
    }

    pub(crate) fn any_backward_conns(&self, f: impl FnMut(&Rc<Connection>) -> bool) -> bool {
        self.backward.borrow().iter().any(f)
    }

    pub(crate) fn mutate_activation(&self, rng: &mut impl Rng, config: &Config) {
        let random_activation = config.activations().choose(rng).cloned().unwrap();
        self.activation.replace(random_activation);
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = f.debug_struct("Node");

        output.field("addr", &format_args!("{:?}", self as *const Self));
        output.field("kind", &self.kind);

        if self.kind != NodeKind::Output {
            output.field("forward", &self.forward.borrow().iter().map(|conn| format!("{:p}", *conn)).collect::<Vec<_>>());
        }

        if self.kind != NodeKind::Input {
            output.field("backward", &self.backward.borrow().iter().map(|conn| format!("{:p}", *conn)).collect::<Vec<_>>());
        }

        output.field("innov", &self.innovation);

        output.finish()
    }
}

impl Eq for Node {}

impl hash::Hash for Node {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.forward.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
        self.backward.borrow().iter().for_each(|node| Rc::as_ptr(node).hash(state));
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind == other.kind {
            self.num_backward_conns().cmp(&other.num_backward_conns()).reverse()
        } else {
            self.kind.cmp(&other.kind).reverse()
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind &&
        self.forward.borrow().iter().zip(other.forward.borrow().iter()).all(|(a, b)| Rc::ptr_eq(a, b)) &&
        self.backward.borrow().iter().zip(other.backward.borrow().iter()).all(|(a, b)| Rc::ptr_eq(a, b))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum NodeKind {
    Input,
    #[default] Hidden,
    Output
}

