use crate::{node::*, population::Population};
use std::{cell::Cell, cmp::Ordering, fmt, hash};

pub struct Conn<'g> {
    leading: Leading<'g>,
    trailing: Trailing<'g>,
    level: usize,
    weight: f32,
    enabled: Cell<bool>,
    innov: usize,
}

impl<'g> Conn<'g> {
    pub fn new(leading: &Leading<'g>, trailing: &Trailing<'g>) -> Self {
        assert_ne!(leading, trailing);
        trailing.update_level(leading.level() + 1);
        Self {
            leading: *leading,
            trailing: *trailing,
            level: leading.level(),
            weight: f32::NAN,
            enabled: true.into(),
            innov: Population::next_conn_innov(leading, trailing)
        }
    }

    pub const fn leading(&self) -> &Leading {
        &self.leading
    }

    pub const fn trailing(&self) -> &Trailing {
        &self.trailing
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

impl Eq for Conn<'_> {}

impl fmt::Debug for Conn<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection")
            .field("Leading Node", &match self.leading {
                Leading::Input(input) => (input as *const _) as *const (),
                Leading::Hidden(hidden) => (hidden as *const _) as *const (),
            })
            .field("Trailing Node", &match self.trailing {
                Trailing::Hidden(hidden) => (hidden as *const _) as *const (),
                Trailing::Output(output) => (output as *const _) as *const (),
            })
            .field("Level", &self.level)
            .field("Weight", &self.weight)
            .field("Enabled", &self.enabled.get())
            .field("Innovation", &self.innov)
            .finish()
    }
}

impl hash::Hash for Conn<'_> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl Ord for Conn<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level)
    }
}

impl PartialEq for Conn<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}

impl PartialOrd for Conn<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

