
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

#[derive(Debug, Clone)]
pub struct Node<A: Scalar> {
    value: Option<Value<A>>,
    prop: Property,
}

impl<A: Scalar> Node<A> {
    fn new(prop: Property) -> Self {
        Self { value: None, prop }
    }
}

#[derive(Debug, Clone, IntoEnum)]
pub enum Property {
    Variable(Variable),
    BinaryOperator(BinaryOperator),
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
}

impl Variable {
    fn new(name: &str) -> Self {
        Variable { name: name.to_string() }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    ScalarMul,
    InnerProd,
    ElementWiseProd,
}

impl BinaryOperator {
    fn exec<A: Scalar>(&self, lhs: &Value<A>, _rhs: &Value<A>) -> Value<A> {
        // TODO implement
        lhs.clone()
    }
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

    fn get_two_arguments(&mut self, binop_node: NodeIndex) -> (NodeIndex, NodeIndex) {
        let mut iter = self.neighbors_directed(binop_node, Direction::Incoming);
        let rhs = iter.next().unwrap();
        let lhs = iter.next().unwrap();
        (rhs, lhs)
    }

    fn get_value(&self, node: NodeIndex) -> Option<&Value<A>> {
        self[node].value.as_ref()
    }

    pub fn eval(&mut self, node: NodeIndex, use_cached: bool) {
        let n = self[node].clone();
        if use_cached && n.value.is_some() {
            return;
        }
        match n.prop {
            Property::Variable(ref v) => {
                panic!("Variable '{}' is evaluated before set value", v.name)
            }
            Property::BinaryOperator(ref op) => {
                let (rhs, lhs) = self.get_two_arguments(node);
                self.eval(rhs, use_cached);
                self.eval(lhs, use_cached);
                let res = op.exec(self.get_value(lhs).unwrap(), self.get_value(rhs).unwrap());
                self[node].value = Some(res);
            }
        };
    }
}
