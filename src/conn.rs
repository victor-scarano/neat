use crate::{node::{Leading, Node, Trailing, Trailable}, pop::Pop};
use core::{cell::Cell, cmp::Ordering, fmt, hash};

pub struct Conn {
    pub innov: usize,
    pub level: usize,
    pub enabled: Cell<bool>,
    pub weight: f32,
    pub leading: Leading,
    pub trailing: Trailing,
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
            weight: f32::NAN,
            leading,
            trailing,
        }
    }
}

impl Eq for Conn {}

impl fmt::Debug for Conn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection")
            .field_with("Leading Node", |f| match &self.leading {
                Leading::Input(input) => fmt::Pointer::fmt(&input, f),
                Leading::Hidden(hidden) => fmt::Pointer::fmt(&hidden, f)
            })
            .field_with("Trailing Node", |f| match &self.trailing {
                Trailing::Hidden(hidden) => fmt::Pointer::fmt(&hidden, f),
                Trailing::Output(output) => fmt::Pointer::fmt(&output, f),
            })
            .field("Level", &self.level)
            .field("Weight", &self.weight)
            .field("Enabled", &self.enabled.get())
            .field("Innovation", &self.innov)
            .finish()
    }
}

impl hash::Hash for Conn {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!();
    }
}

impl Ord for Conn {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level)
    }
}

impl PartialEq for Conn {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}

impl PartialOrd for Conn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

