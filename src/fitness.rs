use crate::genome::Genome;
use core::{mem, ops::Deref};
use rand::Rng;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Fitness(f32);

impl Fitness {
    pub fn rand_parent<'a, const I: usize, const O: usize>(
        mut lhs: &'a Genome<'a, I, O>,
        mut rhs: &'a Genome<'a, I, O>,
        rng: &mut impl Rng
    ) -> &'a Genome<'a, I, O> {
        // this will later be a field in the pop struct
        const MATCHING_PREFERENCE: f64 = 2.0 / 3.0;

        if lhs.fitness > rhs.fitness {
            mem::swap(&mut lhs, &mut rhs);
        }

        let choice = match lhs.fitness == rhs.fitness {
            false => rng.random_bool(MATCHING_PREFERENCE),
            true => rng.random(),
        };

        match choice { false => lhs, true => rhs }
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

