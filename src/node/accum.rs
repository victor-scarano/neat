pub enum Accum {
    Elems(Vec<f32>),
    Eval(f32),
}

impl Accum {
    pub fn new() -> Self {
        Self::Elems(Vec::new())
    }

    pub fn push(&mut self, value: f32) {
        match self {
            Self::Elems(elems) => elems.push(value),
            Self::Eval(_) => panic!(),
        }
    }

    pub fn eval(&mut self, f: fn(&[f32]) -> f32) -> f32 {
        match self {
            Self::Elems(elems) => {
                let eval = f(elems);
                *self = Self::Eval(eval);
                eval
            }
            Self::Eval(eval) => *eval
        }
    }
}

