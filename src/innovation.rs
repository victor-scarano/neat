use std::{cell::{Cell, RefCell}, collections::HashMap, rc::Rc};
use crate::Node;

/// A possible problem is that the same structural innovation will receive different innovation numbers in the same
/// generation if it occurs by chance more than once. However, by keeping a list of the innovations that occurred in
/// the current generation, it is possible to ensure that when the same structure arises more than once through
/// independent mutations in the same generation, each identical mutation is assigned the same innovation number. Thus,
/// there is no resultant explosion of innovation numbers.
#[derive(Debug, Default)]
pub(crate) struct Innovation {
	conns: RefCell<HashMap<(u32, u32), u32>>,
	nodes: Cell<u32>,
}

impl Innovation {
	pub(crate) fn new_conn(&self, input: Rc<Node>, output: Rc<Node>) -> u32 {
		let key = (input.innovation(), output.innovation());
		let len = self.conns.borrow().len() as u32;
		*self.conns.borrow_mut().entry(key).or_insert(len)
	}

	pub(crate) fn new_node(&self) -> u32 {
		let innov = self.nodes.take();
		self.nodes.set(innov + 1);
		innov
	}
}
