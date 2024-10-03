mod conn_input;
mod conn_output;
mod input;
mod hidden;
mod output;

pub(crate) use conn_input::ConnInput;
pub(crate) use conn_output::ConnOutput;
pub(crate) use input::Input;
pub(crate) use hidden::Hidden;
pub(crate) use output::Output;

pub(crate) trait Node {
    fn bias(&self) -> f32;
    fn innov(&self) -> usize;
}

pub(crate) trait ConnInputable {}

pub(crate) trait ConnOutputable {
    fn level(&self) -> usize;
    fn activate(&self, x: f32) -> f32;
}
