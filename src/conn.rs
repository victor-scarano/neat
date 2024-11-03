use crate::{node::*, pop::Pop};
use core::{cell::Cell, cmp::Ordering, fmt, hash};

pub struct Conn {
    pub leading: Leading,
    pub trailing: Trailing,
    pub weight: f32,
    pub enabled: Cell<bool>,
    pub level: usize,
    pub innov: usize,
}

impl Conn {
    pub fn new(leading: impl Into<Leading>, trailing: impl Into<Trailing>) -> Self {
        let leading = leading.into();
        let trailing = trailing.into();

        assert_ne!(leading, trailing);

        trailing.update_level(leading.level() + 1);

        Self {
            innov: Pop::next_conn_innov(&leading, &trailing),
            level: leading.level(),
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
            .field("level", &self.level)
            .field("innov", &self.innov)
            .finish()
    }
}

impl hash::Hash for Conn {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!();
    }
}

// needs to be changed, but this works for now
impl Ord for Conn {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level).then(self.innov.cmp(&other.innov))
    }
}

// used to be equal if innovations were equal, but needs to reflect ord impl
impl PartialEq for Conn {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd for Conn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

