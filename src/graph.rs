
use ndarray::*;
use ndarray_linalg::*;
use petgraph;
use petgraph::prelude::*;

#[derive(Debug, IntoEnum)]
pub enum Node<A: Scalar> {
    Variable(Variable<A>),
    Operator(Operator),
}

#[derive(Debug)]
pub enum Variable<A: Scalar> {
    Empty,
    Scalar(A),
    Vector(Array<A, Ix1>),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    ScalarMul,
    InnerProd,
    ElementWiseProd,
}

#[derive(Debug)]
pub enum Edge<A: Scalar> {
    Empty,
    Scalar(A),
    Vector(Array<A, Ix1>),
    Matrix(Array<A, Ix2>),
}

impl<A: Scalar> Default for Variable<A> {
    fn default() -> Self {
        Variable::Empty
    }
}

impl<A: Scalar> Default for Edge<A> {
    fn default() -> Self {
        Edge::Empty
    }
}

#[derive(Debug, NewType)]
pub struct Graph<A: Scalar>(petgraph::graph::Graph<Node<A>, Edge<A>>);

impl<A: Scalar> Graph<A> {
    pub fn new() -> Self {
        petgraph::graph::Graph::new().into()
    }

    pub fn variable(&mut self) -> NodeIndex {
        self.add_node(Node::Variable(Variable::Empty))
    }

    pub fn plus(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let p = self.add_node(Node::Operator(Operator::Plus));
        self.add_edge(lhs, p, Edge::Empty);
        self.add_edge(rhs, p, Edge::Empty);
        p
    }
}