//! Calculation graph
//!
//! # Examples
//!
//! Create a graph for `(x + y) - z`
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
//! // z = -3.0
//! let z = g.scalar_variable("z", -3.0);
//! // sum = tmp - z
//! let sum = g.sub(tmp, z);
//!
//! g.eval(sum, false).unwrap();
//! let result = g.get_value(sum).unwrap().as_scalar().unwrap();
//! assert!((result - 6.0).abs() < 1e-7);
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

impl<A: Scalar> Value<A> {
    pub fn as_scalar(&self) -> Result<A> {
        match *self {
            Value::Scalar(a) => Ok(a),
            _ => Err(CastError {}.into()),
        }
    }

    pub fn identity() -> Self {
        Value::Scalar(A::from_f64(1.0))
    }
}

#[derive(Debug, Clone)]
pub struct Node<A: Scalar> {
    value: Option<Value<A>>,
    deriv: Option<Value<A>>,
    prop: Property,
}

impl<A: Scalar> Node<A> {
    fn new(prop: Property) -> Self {
        Self {
            value: None,
            deriv: None,
            prop,
        }
    }
}

#[derive(Debug, Clone, IntoEnum)]
enum Property {
    Variable(Variable),
    UnaryOperator(UnaryOperator),
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
pub enum UnaryOperator {
    Negate,
}

impl UnaryOperator {
    fn exec<A: Scalar>(&self, arg: &Value<A>) -> Result<Value<A>> {
        match self {
            &UnaryOperator::Negate => {
                match arg {
                    &Value::Scalar(a) => Ok((-a).into()),
                    &Value::Vector(ref a) => Ok((-a.to_owned()).into()),
                    &Value::Matrix(ref a) => Ok((-a.to_owned()).into()),
                }
            }
        }

    }

    fn eval_deriv<A: Scalar>(&self, arg_last: &Value<A>, deriv: &Value<A>) -> Result<Value<A>> {
        // TODO implmeent
        Ok(Value::identity())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
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

    fn eval_deriv<A: Scalar>(
        &self,
        lhs_last: &Value<A>,
        rhs_last: &Value<A>,
        deriv: &Value<A>,
    ) -> Result<(Value<A>, Value<A>)> {
        // TODO implmeent
        Ok((Value::identity(), Value::identity()))
    }
}

/// Calculation graph based on `petgraph::graph::Graph`
#[derive(Debug, NewType)]
pub struct Graph<A: Scalar>(petgraph::graph::Graph<Node<A>, ()>);

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
        self.add_edge(lhs, p, ());
        self.add_edge(rhs, p, ());
        p
    }

    pub fn negate(&mut self, arg: NodeIndex) -> NodeIndex {
        let neg = UnaryOperator::Negate;
        let n = self.add_node(Node::new(neg.into()));
        self.add_edge(arg, n, ());
        n
    }

    pub fn sub(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let m_rhs = self.negate(rhs);
        self.plus(lhs, m_rhs)
    }

    fn get_arg1(&mut self, op: NodeIndex) -> NodeIndex {
        let mut iter = self.neighbors_directed(op, Direction::Incoming);
        iter.next().unwrap()
    }

    fn get_arg2(&mut self, op: NodeIndex) -> (NodeIndex, NodeIndex) {
        let mut iter = self.neighbors_directed(op, Direction::Incoming);
        let rhs = iter.next().unwrap();
        let lhs = iter.next().unwrap();
        (lhs, rhs)
    }

    /// Get the value of the node. If the value has not been caluclated,
    /// returns `None`
    pub fn get_value(&self, node: NodeIndex) -> Option<&Value<A>> {
        self[node].value.as_ref()
    }

    /// Get the value of the node. If the value has not been caluclated,
    /// returns `None`
    pub fn get_deriv(&self, node: NodeIndex) -> Option<&Value<A>> {
        self[node].deriv.as_ref()
    }

    /// Evaluate the value of the node recusively.
    ///
    /// * `use_cached` - Use the value if already calculated.
    pub fn eval(&mut self, node: NodeIndex, use_cached: bool) -> Result<()> {
        let prop = self[node].prop.clone();
        let value_exists = self[node].value.is_some();
        match prop {
            Property::Variable(ref v) => {
                if value_exists {
                    return Ok(());
                }
                panic!("Variable '{}' is evaluated before set value", v.name)
            }
            Property::UnaryOperator(ref op) => {
                if use_cached && value_exists {
                    return Ok(());
                }
                let arg = self.get_arg1(node);
                self.eval(arg, use_cached)?;
                let res = op.exec(self.get_value(arg).unwrap())?;
                self[node].value = Some(res);
            }
            Property::BinaryOperator(ref op) => {
                if use_cached && value_exists {
                    return Ok(());
                }
                let (lhs, rhs) = self.get_arg2(node);
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

    fn deriv_recur(&mut self, node: NodeIndex, der: Value<A>) -> Result<()> {
        self[node].deriv = Some(der);
        let prop = self[node].prop.clone();
        match prop {
            Property::Variable(ref v) => {}
            Property::UnaryOperator(ref op) => {
                let arg = self.get_arg1(node);
                let der = op.eval_deriv(
                    self.get_value(arg).unwrap(),
                    self.get_deriv(node).unwrap(),
                )?;
                self.deriv_recur(arg, der)?;
            }
            Property::BinaryOperator(ref op) => {
                let (lhs, rhs) = self.get_arg2(node);
                let (l_der, r_der) = op.eval_deriv(
                    self.get_value(lhs).unwrap(),
                    self.get_value(rhs).unwrap(),
                    self.get_deriv(node).unwrap(),
                )?;
                self.deriv_recur(lhs, l_der)?;
                self.deriv_recur(rhs, r_der)?;
            }
        };
        Ok(())
    }

    pub fn deriv(&mut self, node: NodeIndex) -> Result<()> {
        self.deriv_recur(node, Value::identity())
    }
}
