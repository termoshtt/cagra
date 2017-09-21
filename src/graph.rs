
use ndarray::*;
use ndarray_linalg::*;
use petgraph;
use petgraph::prelude::*;

#[derive(Debug, IntoEnum)]
pub enum Node<A: Scalar> {
    Variable(Variable<A>),
    BinaryOperator(BinaryOperator<A>),
}

#[derive(Debug)]
pub struct Variable<A: Scalar> {
    pub name: String,
    pub value: Option<Value<A>>,
    pub generation: usize,
}

impl<A: Scalar> Variable<A> {
    pub fn new(name: &str) -> Self {
        Variable {
            name: name.to_string(),
            value: None,
            generation: 0,
        }
    }
}

#[derive(Debug)]
pub struct Edge<A: Scalar> {
    pub value: Option<Value<A>>,
    pub generation: usize,
}

impl<A: Scalar> Edge<A> {
    pub fn new() -> Self {
        Edge {
            value: None,
            generation: 0,
        }
    }
}

#[derive(Debug, Clone, IntoEnum)]
pub enum Value<A: Scalar> {
    Scalar(A),
    Vector(Array<A, Ix1>),
    Matrix(Array<A, Ix2>),
}

#[derive(Debug)]
pub struct BinaryOperator<A: Scalar> {
    value: Option<Value<A>>,
    ty: BinaryOpType,
    lhs: NodeIndex,
    rhs: NodeIndex,
}

#[derive(Debug)]
pub enum BinaryOpType {
    Plus,
    ScalarMul,
    InnerProd,
    ElementWiseProd,
}

#[derive(Debug, NewType)]
pub struct Graph<A: Scalar>(petgraph::graph::Graph<Node<A>, Edge<A>>);

impl<A: Scalar> Graph<A> {
    pub fn new() -> Self {
        petgraph::graph::Graph::new().into()
    }

    pub fn variable(&mut self, name: &str) -> NodeIndex {
        self.add_node(Variable::new(name).into())
    }

    pub fn plus(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let plus = BinaryOperator {
            value: None,
            ty: BinaryOpType::Plus,
            lhs,
            rhs,
        };
        let p = self.add_node(plus.into());
        self.add_edge(lhs, p, Edge::new());
        self.add_edge(rhs, p, Edge::new());
        p
    }

    pub fn eval(&mut self, node: NodeIndex) -> Value<A> {
        let n = &self[node];
        match n {
            &Node::Variable(ref v) => v.value.clone().expect("value is empty"),
            _ => unimplemented!("Operation is not implemented"),
        }
    }
}
