
extern crate alloc;
use crate::{node::*, pop::Pop};
use core::{cell::Cell, cmp::Ordering, fmt, hash};
use alloc::{collections::BTreeSet, rc::Rc};
use hashbrown::HashSet;

#[derive(Clone)]
pub struct Conn {
    pub leading: Leading,
    pub trailing: Trailing,
    pub weight: f32,
    pub enabled: Cell<bool>,
    pub layer: usize,
    pub innov: usize,
}

impl Conn {
    pub fn new(leading: impl Into<Leading>, trailing: impl Into<Trailing>) -> Self {
        let leading = leading.into();
        let trailing = trailing.into();

        assert_ne!(leading, trailing);

        trailing.update_layer(leading.layer() + 1);

        Self {
            innov: Pop::next_conn_innov(&leading, &trailing),
            layer: leading.layer(),
            enabled: true.into(),
            weight: 1.0,
            leading,
            trailing,
        }
    }
}

impl Eq for Conn {}

impl fmt::Debug for Conn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f
            .debug_struct("Conn")
            .field_with("leading", |f| match &self.leading {
                Leading::Input(input) => fmt::Pointer::fmt(input, f),
                Leading::Hidden(hidden) => fmt::Pointer::fmt(hidden, f)
            })
            .field_with("trailing", |f| match &self.trailing {
                Trailing::Hidden(hidden) => fmt::Pointer::fmt(hidden, f),
                Trailing::Output(output) => fmt::Pointer::fmt(output, f),
            })
            .field("weight", &self.weight)
            .field("enabled", &self.enabled.get())
            .field("layer", &self.layer)
            .field("innov", &self.innov)
            .finish()
    }
}

impl hash::Hash for Conn {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.innov.hash(state);
    }
}

impl Ord for Conn {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.enabled.get()
        //    .cmp(&other.enabled.get())
        //    .reverse()
        self.layer.cmp(&other.layer).then(self.innov.cmp(&other.innov))
    }
}

// used to be equal if innovations were equal, but needs to reflect ord impl
impl PartialEq for Conn {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq() && self.innov == other.innov
    }
}

impl PartialOrd for Conn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// TODO: Write custom Rc implementation to optimize for only two possible references so that the RcInner allocation
// isn't as large as it normally is
#[derive(Default)]
pub struct Conns {
    btree: BTreeSet<Rc<Conn>>,
    hash: HashSet<Rc<Conn>>,
}

impl Conns {
    pub fn from_matching_disjoint(matching: Vec<&Conn>, disjoint: Vec<&Conn>) -> Self {
        let btree = BTreeSet::from_iter(matching.into_iter()
            .chain(disjoint.into_iter())
            .map(|conn| Rc::new(conn.clone())));

        let hash = HashSet::from_iter(btree.iter().cloned());

        assert_eq!(btree.len(), hash.len());

        Self { btree, hash }
    }

    pub fn get(&self, conn: &Conn) -> &Conn {
        self.hash.get(conn).unwrap()
    }

    pub fn insert(&mut self, conn: Conn) {
        let conn = Rc::new(conn);

        let inserted = self.btree.insert(conn.clone());
        assert!(inserted);

        let inserted = self.hash.insert(conn);
        assert!(inserted);
    }

    pub fn iter_ordered(&self) -> impl Iterator<Item = &Conn> {
        self.btree.iter().map(<Rc<Conn> as AsRef<Conn>>::as_ref)
    }

    pub fn iter_unordered(&self) -> impl Iterator<Item = &Conn> {
        self.hash.iter().map(<Rc<Conn> as AsRef<Conn>>::as_ref)
    }

    pub fn hash_difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Conn> {
        self.hash.difference(&other.hash).map(<Rc<Conn> as AsRef<Conn>>::as_ref)
    }

    pub fn hash_intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Conn> {
        self.hash
            .intersection(&other.hash)
            .map(<Rc<Conn> as AsRef<Conn>>::as_ref)
    }

    pub fn hash_symmetric_difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Conn> {
        self.hash
            .symmetric_difference(&other.hash)
            .map(<Rc<Conn> as AsRef<Conn>>::as_ref)
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.btree.len(), self.hash.len());
        self.hash.len()
    }
}

