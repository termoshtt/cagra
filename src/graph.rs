
use ndarray::*;
use ndarray_linalg::*;
use petgraph;
use petgraph::prelude::*;

#[derive(Debug, Clone, IntoEnum)]
pub enum Value<A: Scalar> {
    Scalar(A),
    Vector(Array<A, Ix1>),
    Matrix(Array<A, Ix2>),
}

#[derive(Debug)]
pub struct Node<A: Scalar> {
    value: Option<Value<A>>,
    prop: Property,
}

impl<A: Scalar> Node<A> {
    fn new(prop: Property) -> Self {
        Self { value: None, prop }
    }
}

#[derive(Debug, IntoEnum)]
pub enum Property {
    Variable(Variable),
    BinaryOperator(BinaryOperator),
}

#[derive(Debug)]
pub struct Variable {
    name: String,
}

impl Variable {
    fn new(name: &str) -> Self {
        Variable { name: name.to_string() }
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    ScalarMul,
    InnerProd,
    ElementWiseProd,
}

#[derive(Debug)]
pub struct Edge<A: Scalar> {
    value: Option<Value<A>>,
}

impl<A: Scalar> Edge<A> {
    fn new() -> Self {
        Edge { value: None }
    }
}

#[derive(Debug, NewType)]
pub struct Graph<A: Scalar>(petgraph::graph::Graph<Node<A>, Edge<A>>);

impl<A: Scalar> Graph<A> {
    pub fn new() -> Self {
        petgraph::graph::Graph::new().into()
    }

    pub fn variable(&mut self, name: &str) -> NodeIndex {
        let var = Variable::new(name);
        self.add_node(Node::new(var.into()))
    }

    pub fn set_value<V: Into<Value<A>>>(&mut self, node: NodeIndex, value: V) {
        self[node].value = Some(value.into());
    }

    pub fn plus(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let plus = BinaryOperator::Plus;
        let p = self.add_node(Node::new(plus.into()));
        self.add_edge(lhs, p, Edge::new());
        self.add_edge(rhs, p, Edge::new());
        p
    }

    fn eval(&mut self, node: NodeIndex, use_cached: bool) {
        let n = &self[node];
        if use_cached && n.value.is_some() {
            return;
        }
        match n.prop {
            Property::Variable(ref v) => {
                panic!("Variable '{}' is evaluated before set value", v.name)
            }
            Property::BinaryOperator(_) => {} // TODO eval value recursively
        };
    }
}
