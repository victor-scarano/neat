use crate::{node::*, pop::Pop};
use core::{cell::Cell, cmp::Ordering, fmt, hash};

pub struct Conn {
    pub leading: Leading,
    pub trailing: Trailing,
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
            leading: leading.clone(),
            trailing: trailing.clone(),
            level: leading.level(),
            weight: f32::NAN,
            enabled: true.into(),
            innov: Pop::next_conn_innov(&leading, &trailing)
        }
    }

    pub fn leading(&self) -> Leading {
        self.leading.clone()
    }

    pub fn trailing(&self) -> Trailing {
        self.trailing.clone()
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

