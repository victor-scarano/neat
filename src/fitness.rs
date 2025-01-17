use crate::genome::Genome;
use core::{mem, ops::Deref};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Fitness(f32);

impl Fitness {
    pub fn rand_parent<'a, const I: usize, const O: usize>(
        mut lhs: &'a Genome<I, O>,
        mut rhs: &'a Genome<I, O>,
        rng: &mut impl Rng
    ) -> &'a Genome<I, O> {
        // this will later be a field in the pop struct
        const MATCHING_PREFERENCE: f64 = 2.0 / 3.0;

        if lhs.fitness > rhs.fitness {
            mem::swap(&mut lhs, &mut rhs);
        }

        let choice = match lhs.fitness == rhs.fitness {
            false => rng.gen_bool(MATCHING_PREFERENCE),
            true => rng.gen(),
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

