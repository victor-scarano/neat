use crate::{node::*, population::Population};
use std::{cell::Cell, cmp::Ordering, fmt, hash};

pub struct Conn {
    pub leading: UnsafeLeading,
    pub trailing: UnsafeTrailing,
    level: usize,
    weight: f32,
    enabled: Cell<bool>,
    innov: usize,
}

impl Conn {
    pub fn new(leading: Leading, trailing: Trailing) -> Self {
        assert_ne!(leading, trailing);
        trailing.update_level(leading.level() + 1);
        Self {
            leading: UnsafeLeading::from(leading),
            trailing: UnsafeTrailing::from(trailing),
            level: leading.level(),
            weight: f32::NAN,
            enabled: true.into(),
            innov: Population::next_conn_innov(&leading, &trailing)
        }
    }

    pub fn leading(&self) -> Leading {
        Leading::from(&self.leading)
    }

    pub fn trailing(&self) -> Trailing {
        Trailing::from(&self.trailing)
    }

    pub const fn level(&self) -> usize {
        self.level
    }

    pub const fn weight(&self) -> f32 {
        self.weight
    }

    pub fn enabled(&self) -> bool {
        self.enabled.get()
    }

    pub const fn innov(&self) -> usize {
        self.innov
    }

    pub fn disable(&self) {
        self.enabled.set(false);
    }
}

impl Eq for Conn {}

impl fmt::Debug for Conn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection")
            .field_with("Leading Node", |f| match self.leading {
                UnsafeLeading::Input(input) => fmt::Pointer::fmt(&input, f),
                UnsafeLeading::Hidden(hidden) => fmt::Pointer::fmt(&hidden, f)
            })
            .field_with("Trailing Node", |f| match self.trailing {
                UnsafeTrailing::Hidden(hidden) => fmt::Pointer::fmt(&hidden, f),
                UnsafeTrailing::Output(output) => fmt::Pointer::fmt(&output, f),
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
        todo!()
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

