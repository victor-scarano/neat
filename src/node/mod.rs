mod input;
mod hidden;
mod leading;
mod output;
mod trailing;

pub use input::Input;
pub use hidden::Hidden;
pub use leading::Leading;
pub use output::Output;
pub use trailing::Trailing;

pub trait Node {
    fn level(&self) -> usize;
    fn bias(&self) -> f32;
    fn innov(&self) -> usize;
}

pub trait Trailable {
    fn update_level(&self, level: usize);
    fn activate(&self, x: f32) -> f32;
    fn response(&self) -> f32;
}
