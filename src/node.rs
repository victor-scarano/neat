use crate::Connection;
use std::{cell::RefCell, cmp::Ordering, collections::BTreeSet, fmt, hash, rc::Rc};

#[derive(Default, Clone, Eq)]
pub(crate) struct Node {
    kind: NodeKind,
    forward: RefCell<BTreeSet<Rc<Connection>>>,
    backward: RefCell<BTreeSet<Rc<Connection>>>,
    innovation: u32,
}

impl Node {
    pub(crate) fn new_input(innov: u32) -> Self {
        Self {
            kind: NodeKind::Input,
            forward: RefCell::new(BTreeSet::new()),
            backward: RefCell::new(BTreeSet::new()),
            innovation: innov,
        }
    }

    pub(crate) fn new_hidden(innov: u32) -> Self {
        Self {
            kind: NodeKind::Hidden,
            forward: RefCell::new(BTreeSet::new()),
            backward: RefCell::new(BTreeSet::new()),
            innovation: innov,
        }
    }

    pub(crate) fn new_output(innov: u32) -> Self {
        Self {
            kind: NodeKind::Output,
            forward: RefCell::new(BTreeSet::new()),
            backward: RefCell::new(BTreeSet::new()),
            innovation: innov,
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

/// Specifies the position of a [`Node`] in a genome.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum NodeKind {
    /// Represents an input node of a genome.
    Input,

    /// Represents a hidden node of a genome.
    #[default]
    Hidden,

    /// Represents an output node of a genome.
    Output
}

