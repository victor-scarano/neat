mod conn_input;
mod conn_output;
mod input;
mod hidden;
mod output;

pub use conn_input::ConnInput;
pub use conn_output::ConnOutput;
pub use input::Input;
pub use hidden::Hidden;
pub use output::Output;

pub trait Node {
    fn level(&self) -> usize;
    fn bias(&self) -> f32;
    fn innov(&self) -> usize;
}

pub trait ConnInputable {}

pub trait ConnOutputable {
    fn update_level(&self, level: usize);
    fn activate(&self, x: f32) -> f32;
    fn response(&self) -> f32;
}
