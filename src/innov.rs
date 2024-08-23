use crate::node::Node;
use std::{cell::{Cell, RefCell}, collections::HashMap, rc::Rc};

#[derive(Debug, Default)]
pub(crate) struct Innov {
	conns: RefCell<HashMap<(u32, u32), u32>>,
	nodes: Cell<u32>,
}

impl Innov {
	pub(crate) fn new_conn_innovation(&self, input: Rc<dyn Node>, output: Rc<dyn Node>) -> u32 {
		let key = (input.innovation(), output.innovation());
		let len = self.conns.borrow().len() as u32;
		*self.conns.borrow_mut().entry(key).or_insert(len)
	}

	pub(crate) fn new_node_innovation(&self) -> u32 {
		let innov = self.nodes.take();
		self.nodes.set(innov + 1);
		innov
	}
}

