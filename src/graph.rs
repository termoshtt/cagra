//! Calculation graph
//!
//! # Examples
//!
//! Create a graph for `(x + y) + z`
//!
//! ```
//! use cagra::graph::*;
//!
//! let mut g: Graph<f64> = Graph::new();
//! // x = 1.0
//! let x = g.scalar_variable("x", 1.0);
//! // y = 2.0
//! let y = g.scalar_variable("y", 2.0);
//! // tmp = x + y
//! let tmp = g.plus(x, y);
//! // z = 3.0
//! let z = g.scalar_variable("z", 3.0);
//! // sum = tmp + z
//! let sum = g.plus(tmp, z);
//!
//! g.eval(sum, false).unwrap();
//! println!("(x + y) + z = {:?}", g.get_value(sum).unwrap());  // 6.0
//! ```

use ndarray::*;
use ndarray_linalg::*;
use petgraph;
use petgraph::prelude::*;

use error::*;

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
enum Property {
    Variable(Variable),
    BinaryOperator(BinaryOperator),
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
}

impl Variable {
    fn new(name: &str) -> Self {
        Variable { name: name.to_string() }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BinaryOperator {
    Plus,
}

impl BinaryOperator {
    fn exec<A: Scalar>(&self, lhs: &Value<A>, rhs: &Value<A>) -> Result<Value<A>> {
        match self {
            &BinaryOperator::Plus => {
                match (lhs, rhs) {
                    (&Value::Scalar(ref l), &Value::Scalar(ref r)) => Ok((*l + *r).into()),
                    (&Value::Vector(ref l), &Value::Vector(ref r)) => Ok((l + r).into()),
                    (&Value::Matrix(ref l), &Value::Matrix(ref r)) => Ok((l + r).into()),
                    _ => Err(BinOpTypeError { op: *self }.into()),
                }
            }
        }
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

/// Calculation graph based on `petgraph::graph::Graph`
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

    pub fn scalar_variable(&mut self, name: &str, value: A) -> NodeIndex {
        let var = self.variable(name);
        self.set_value(var, value);
        var
    }

    pub fn vector_variable(&mut self, name: &str, value: Array<A, Ix1>) -> NodeIndex {
        let var = self.variable(name);
        self.set_value(var, value);
        var
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

    /// Get the value of the node. If the value has not been caluclated,
    /// returns `None`
    pub fn get_value(&self, node: NodeIndex) -> Option<&Value<A>> {
        self[node].value.as_ref()
    }

    /// Evaluate the value of the node recusively.
    ///
    /// * `use_cached` - Use the value if already calculated.
    pub fn eval(&mut self, node: NodeIndex, use_cached: bool) -> Result<()> {
        let n = self[node].clone();
        match n.prop {
            Property::Variable(ref v) => {
                if n.value.is_some() {
                    return Ok(());
                }
                panic!("Variable '{}' is evaluated before set value", v.name)
            }
            Property::BinaryOperator(ref op) => {
                if use_cached && n.value.is_some() {
                    return Ok(());
                }
                let (rhs, lhs) = self.get_two_arguments(node);
                self.eval(rhs, use_cached)?;
                self.eval(lhs, use_cached)?;
                let res = op.exec(
                    self.get_value(lhs).unwrap(),
                    self.get_value(rhs).unwrap(),
                )?;
                self[node].value = Some(res);
            }
        };
        Ok(())
    }
}
