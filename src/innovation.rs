use crate::Node;
use std::{cell::{Cell, RefCell}, collections::HashMap, rc::Rc};

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

