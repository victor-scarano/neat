use core::ops::Deref;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Fitness(f32);

impl Fitness {
    pub fn gen_bool(lhs: Self, rhs: Self, rng: &mut impl Rng) -> bool {
        // this will later be a field in the pop struct
        const MATCHING_PREFERENCE: f64 = 2.0 / 3.0;

        match lhs == rhs {
            true => rng.gen(),
            false => rng.gen_bool(MATCHING_PREFERENCE)
        }
    }
}

impl Deref for Fitness {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f32> for Fitness {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

