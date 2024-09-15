use crate::Conn;
use rand::Rng;

mod input;

pub(crate) trait Node {
    fn new<R: Rng>(rng: &mut R) -> Self where Self: Sized;
    fn innov(&self) -> usize;
}

pub(crate) trait ConnInput<'genome>: Node {
    // might need to change the name to insert_forward_conn
    fn insert_conn(&self, conn: &'genome Conn<'genome>);
    fn num_conns(&self) -> usize;
    fn iter_conns(&self) -> Box<dyn Iterator<Item = &&'genome Conn<'genome>>>;
}

pub(crate) trait ConnOutput<'genome>: Node {}
