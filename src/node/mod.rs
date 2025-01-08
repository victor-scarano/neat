mod accum;
mod head;
mod hidden;
mod input;
mod output;
mod tail;

pub use accum::Accum;
pub use head::*;
pub use hidden::*;
pub use input::*;
pub use output::*;
pub use tail::*;

pub trait Node {
    fn layer(&self) -> usize;
    fn bias(&self) -> f32;
    fn innov(&self) -> usize;
    fn update_layer(&self, layer: usize);
    fn activate(&self, x: f32) -> f32;
    fn response(&self) -> f32;
    fn aggregator(&self) -> fn(&[f32]) -> f32;
}

