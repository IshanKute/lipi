use super::node::Node;

pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Node>,
}
