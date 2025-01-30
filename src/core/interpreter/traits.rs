use crate::core::ast::node::Node;
use std::error::Error;

pub trait Interpreter {
    fn interpret(&mut self, node: Node) -> Result<(), Box<dyn Error>>;
}
