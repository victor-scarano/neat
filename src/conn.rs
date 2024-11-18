extern crate alloc;
use crate::{node::*, pop::Pop};
use core::{cell::Cell, cmp::Ordering, fmt, hash};
use alloc::{collections::BTreeSet, rc::*};
use hashbrown::HashSet;

#[derive(Clone)]
pub struct Conn<'g> {
    pub tail: Tail<'g>,
    pub head: Head<'g>,
    pub weight: f32,
    pub enabled: Cell<bool>,
    pub layer: usize,
    pub innov: usize,
}

impl Conn<'_> {
    pub fn new(tail: impl Into<Tail>, head: impl Into<Head>) -> Self {
        let tail = tail.into();
        let head = head.into();

        assert_ne!(tail, head);

        head.update_layer(tail.layer() + 1);

        Self {
            innov: Pop::next_conn_innov(&tail, &head),
            layer: tail.layer(),
            enabled: true.into(),
            weight: 1.0,
            tail,
            head,
        }
    }
}

impl Eq for Conn<'_> {}

impl fmt::Debug for Conn<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f
            .debug_struct("Conn")
            .field_with("tail", |f| fmt::Pointer::fmt(&self.tail, f))
            .field_with("head", |f| fmt::Pointer::fmt(&self.head, f))
            .field("weight", &self.weight)
            .field("enabled", &self.enabled.get())
            .field("layer", &self.layer)
            .field("innov", &self.innov)
            .finish()
    }
}

impl hash::Hash for Conn<'_> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.innov.hash(state);
    }
}

impl Ord for Conn<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.enabled.get()
        //    .cmp(&other.enabled.get())
        //    .reverse()
        self.layer.cmp(&other.layer).then(self.innov.cmp(&other.innov))
    }
}

// used to be equal if innovations were equal, but needs to reflect ord impl
impl PartialEq for Conn<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq() && self.innov == other.innov
    }
}

impl PartialOrd for Conn<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// TODO: Write custom Rc implementation to optimize for only two possible references so that the RcInner allocation
// isn't as large as it normally is
#[derive(Default)]
pub struct Conns<'g> {
    btree: BTreeSet<Rc<Conn<'g>>>,
    hash: HashSet<Rc<Conn<'g>>>,
}

impl Conns<'_> {
    // probably optimizable
    // probably a cleaner way to do this
    pub fn clone_from<const I: usize, const O: usize>(
        &self,
        inputs: &[Rc<Input>; I],
        hiddens: &HashSet<Rc<Hidden>>,
        outputs: &[Rc<Output>; O],
    ) -> Self {
        let btree = BTreeSet::from_iter(self.btree.iter()
            .map(<Rc<Conn> as AsRef<Conn>>::as_ref)
            .map(Conn::clone)
            .map(|mut conn| {
                conn.tail = match conn.tail {
                    Tail::Input(ref input) => Tail::Input(inputs[input.idx()].clone()),
                    Tail::Hidden(ref hidden) => Tail::Hidden(hiddens.get(hidden).cloned().unwrap()),
                };

                conn.head = match conn.head {
                    Head::Hidden(ref hidden) => Head::Hidden(hiddens.get(hidden).cloned().unwrap()),
                    Head::Output(ref output) => Head::Output(outputs[output.idx::<I>()].clone()),
                };

                Rc::new(conn)
            }));

        let hash = HashSet::from_iter(btree.iter().cloned());

        assert_eq!(btree.len(), hash.len());

        Self { btree, hash }
    }

    pub fn from(matching: Vec<&Conn>, disjoint: Vec<&Conn>) -> Self {
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

    pub fn insert(&mut self, conn: Conn) -> Weak<Conn> {
        let conn = Rc::new(conn);

        let inserted = self.btree.insert(conn.clone());
        assert!(inserted);

        let inserted = self.hash.insert(conn.clone());
        assert!(inserted);

        Rc::downgrade(&conn)
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

