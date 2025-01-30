use super::traits::Interpreter;
use crate::core::ast::node::Node;
use std::error::Error;

pub struct FunctionInterpreter;

impl FunctionInterpreter {
    pub fn new() -> Self {
        FunctionInterpreter
    }
}

impl Interpreter for FunctionInterpreter {
    fn interpret(&mut self, node: Node) -> Result<(), Box<dyn Error>> {
        match node {
            Node::Print(expr) => {
                match *expr {
                    Node::StringLiteral(s) => println!("{}", s),
                    _ => return Err("Expected string literal".into()),
                }
                Ok(())
            }
            _ => Err("Expected print statement".into()),
        }
    }
}
