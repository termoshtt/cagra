//! Calculation graph

use petgraph::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::{hash_map::Entry, HashMap};
use std::{fmt, io};

use super::error::{Error, Result};
use super::operator::{Binary, Unary};
use cauchy::Scalar;

pub type Tensor<A> = ndarray::ArcArray<A, ndarray::IxDyn>;

#[macro_export]
macro_rules! graph {
    ($scalar:ty, $proc:block) => {{
        // non-hygienic hack
        // See https://qiita.com/ubnt_intrepid/items/dcfabd5b0ae4d4e105da (Japanese)
        enum DummyGraphNew {}
        impl DummyGraphNew {
            $crate::parser::graph_impl!($scalar, $proc);
        }
        DummyGraphNew::graph_new()
    }};
}

/// Node of the calculation graph.
///
/// This struct keeps the last value, and `Graph` calculates the derivative
/// using this value.
#[derive(Clone)]
pub struct Node<A: Scalar> {
    value: Option<Tensor<A>>,
    deriv: Option<Tensor<A>>,
    property: Property,
}

impl<A: Scalar + fmt::Debug> fmt::Debug for Node<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.property {
            Property::Constant | Property::Variable => {}
            Property::Unary(unary) => write!(f, "Unary: {:?}\n", unary)?,
            Property::Binary(bin) => write!(f, "Binary: {:?}\n", bin)?,
        }
        if let Some(val) = &self.value {
            write!(f, "value={:?}", val)?
        } else {
            write!(f, "value=N/A")?
        }
        write!(f, ", ")?;
        if let Some(deriv) = &self.deriv {
            write!(f, "deriv={:?}", deriv)?;
        } else {
            write!(f, "deriv=N/A")?;
        }
        Ok(())
    }
}

/// Extra propaties of the `Node` accoding to the node type.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
enum Property {
    Constant,
    Variable,
    Unary(Unary),
    Binary(Binary),
}

impl<A: Scalar> Node<A> {
    /// Check the node is variable
    pub fn is_variable(&self) -> bool {
        match self.property {
            Property::Constant => false,
            Property::Variable => true,
            Property::Unary(_) => false,
            Property::Binary(_) => false,
        }
    }

    fn variable() -> Self {
        Self {
            value: None,
            deriv: None,
            property: Property::Variable,
        }
    }

    fn constant(a: Tensor<A>) -> Self {
        Self {
            value: Some(a),
            deriv: None,
            property: Property::Constant,
        }
    }
}

impl<A: Scalar> From<Unary> for Node<A> {
    fn from(op: Unary) -> Self {
        Self {
            value: None,
            deriv: None,
            property: Property::Unary(op),
        }
    }
}

impl<A: Scalar> From<Binary> for Node<A> {
    fn from(op: Binary) -> Self {
        Self {
            value: None,
            deriv: None,
            property: Property::Binary(op),
        }
    }
}

/// Calculation graph based on `petgraph::graph::Graph`
#[derive(Debug, Clone)]
pub struct Graph<A: Scalar> {
    graph: petgraph::graph::Graph<Node<A>, ()>,
    namespace: HashMap<String, NodeIndex>,
}

impl<A: Scalar> Graph<A> {
    pub fn get_index(&self, name: &str) -> NodeIndex {
        self.namespace[name]
    }
}

// Panic if the index does not exists
impl<A: Scalar> ::std::ops::Index<NodeIndex> for Graph<A> {
    type Output = Node<A>;
    fn index(&self, index: NodeIndex) -> &Node<A> {
        &self.graph[index]
    }
}

// Panic if the index does not exists
impl<A: Scalar> ::std::ops::IndexMut<NodeIndex> for Graph<A> {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Node<A> {
        &mut self.graph[index]
    }
}

// Panic if the name is not found
impl<A: Scalar> ::std::ops::Index<&str> for Graph<A> {
    type Output = Node<A>;
    fn index(&self, name: &str) -> &Node<A> {
        let index = self.namespace[name];
        &self.graph[index]
    }
}

// Panic if the name is not found
impl<A: Scalar> ::std::ops::IndexMut<&str> for Graph<A> {
    fn index_mut(&mut self, name: &str) -> &mut Node<A> {
        let index = self.namespace[name];
        &mut self.graph[index]
    }
}

macro_rules! def_unary { ($name:ident, $enum:ident) => {
    pub fn $name(&mut self, arg: NodeIndex) -> NodeIndex {
        let n = self.graph.add_node(Unary::$enum.into());
        self.graph.add_edge(arg, n, ());
        n
    }
}} // def_unary

macro_rules! def_binary { ($name:ident, $enum:ident) => {
    pub fn $name(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let p = self.graph.add_node(Binary::$enum.into());
        self.graph.add_edge(lhs, p, ());
        self.graph.add_edge(rhs, p, ());
        p
    }
}} // def_binary

impl<A: Scalar> Graph<A> {
    def_binary!(add, Add);
    def_binary!(mul, Mul);
    def_binary!(div, Div);
    def_binary!(dot, Dot);
    def_unary!(neg, Neg);
    def_unary!(square, Square);
    def_unary!(exp, Exp);
    def_unary!(ln, Ln);
    def_unary!(sin, Sin);
    def_unary!(cos, Cos);
    def_unary!(tan, Tan);
    def_unary!(sinh, Sinh);
    def_unary!(cosh, Cosh);
    def_unary!(tanh, Tanh);

    pub fn sub(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let m_rhs = self.neg(rhs);
        self.add(lhs, m_rhs)
    }

    /// new graph.
    pub fn new() -> Self {
        Self {
            graph: petgraph::graph::Graph::new(),
            namespace: HashMap::new(),
        }
    }

    pub fn constant_scalar(&mut self, value: A) -> NodeIndex {
        let value = ndarray::arr0(value).into_dyn().into_shared();
        self.graph.add_node(Node::constant(value))
    }

    pub fn constant_vector(&mut self, value: &[A]) -> NodeIndex {
        let value = ndarray::arr1(value).into_dyn().into_shared();
        self.graph.add_node(Node::constant(value))
    }

    pub fn constant(&mut self, value: Tensor<A>) -> NodeIndex {
        self.graph.add_node(Node::constant(value))
    }

    /// Create new empty variable
    pub fn empty_variable(&mut self, name: &str) -> Result<NodeIndex> {
        // check name duplication
        match self.namespace.entry(name.into()) {
            Entry::Occupied(_) => Err(Error::DuplicatedName { name: name.into() }),
            Entry::Vacant(entry) => {
                let id = self.graph.add_node(Node::variable());
                entry.insert(id);
                Ok(id)
            }
        }
    }

    /// Create new 0-dimensional variable
    pub fn scalar(&mut self, name: &str, value: A) -> Result<NodeIndex> {
        let value = ndarray::arr0(value).into_dyn().into_shared();
        let var = self.empty_variable(name)?;
        self.set_value(var, value).unwrap();
        Ok(var)
    }

    /// Create new 1-dimensional variable
    pub fn vector(&mut self, name: &str, value: &[A]) -> Result<NodeIndex> {
        let value = ndarray::arr1(value).into_dyn().into_shared();
        let var = self.empty_variable(name)?;
        self.set_value(var, value).unwrap();
        Ok(var)
    }

    /// Create new variable with value
    pub fn variable(&mut self, name: &str, value: Tensor<A>) -> Result<NodeIndex> {
        let var = self.empty_variable(name)?;
        self.set_value(var, value).unwrap();
        Ok(var)
    }

    pub fn set_name(&mut self, node: NodeIndex, name: &str) -> Option<NodeIndex> {
        self.namespace.insert(name.to_string(), node)
    }

    /// Set a value to a variable node, and returns `NodeTypeError` if the node is an operator.
    pub fn set_value(&mut self, node: NodeIndex, value: Tensor<A>) -> Result<()> {
        if self.graph[node].is_variable() {
            self.graph[node].value = Some(value);
            Ok(())
        } else {
            Err(Error::NodeTypeError {
                index: node.index(),
            })
        }
    }

    fn get_arg1(&mut self, op: NodeIndex) -> NodeIndex {
        let mut iter = self.graph.neighbors_directed(op, Direction::Incoming);
        iter.next().unwrap()
    }

    fn get_arg2(&mut self, op: NodeIndex) -> (NodeIndex, NodeIndex) {
        let mut iter = self.graph.neighbors_directed(op, Direction::Incoming);
        let rhs = iter.next().unwrap();
        let lhs = iter.next().unwrap();
        (lhs, rhs)
    }

    /// Evaluate the value of the node recusively.
    pub fn eval_value(&mut self, node: NodeIndex) -> Result<Tensor<A>> {
        let prop = self[node].property;
        match prop {
            Property::Variable => self.get_value(node),
            Property::Constant => Ok(self
                .get_value(node)
                .expect("Constant node is not initialized")),
            Property::Unary(ref op) => Ok(self.get_value(node).unwrap_or({
                let arg = self.get_arg1(node);
                let val1 = self.eval_value(arg)?;
                let value = op.eval_value(val1);
                self[node].value = Some(value.clone()); // cache
                value
            })),
            Property::Binary(ref op) => Ok(self.get_value(node).unwrap_or({
                let (lhs, rhs) = self.get_arg2(node);
                let lv = self.eval_value(lhs)?;
                let rv = self.eval_value(rhs)?;
                let value = op.eval_value(lv, rv);
                self[node].value = Some(value.clone()); // cache
                value
            })),
        }
    }

    pub fn get_value(&self, node: NodeIndex) -> Result<Tensor<A>> {
        self[node].value.clone().ok_or(Error::ValueUninitialized {
            index: node.index(),
        })
    }

    pub fn get_deriv(&self, node: NodeIndex) -> Result<Tensor<A>> {
        self[node].deriv.clone().ok_or(Error::DerivUninitialized {
            index: node.index(),
        })
    }

    fn deriv_recur(&mut self, node: NodeIndex, der: Tensor<A>) -> Result<()> {
        self[node].deriv = match self[node].deriv.clone() {
            Some(der_last) => Some(der_last + der.clone()),
            None => Some(der.clone()),
        };
        let property = self[node].property;
        match property {
            Property::Variable | Property::Constant => {}
            Property::Unary(ref op) => {
                let arg = self.get_arg1(node);
                let der = op.eval_deriv(self.get_value(arg)?, der);
                self.deriv_recur(arg, der)?;
            }
            Property::Binary(ref op) => {
                let (lhs, rhs) = self.get_arg2(node);
                let (l_der, r_der) = op.eval_deriv(self.get_value(lhs)?, self.get_value(rhs)?, der);
                self.deriv_recur(lhs, l_der)?;
                self.deriv_recur(rhs, r_der)?;
            }
        };
        Ok(())
    }

    /// Evaluate derivative recursively.
    pub fn eval_deriv(&mut self, node: NodeIndex) -> Result<()> {
        for idx in self.graph.node_indices() {
            self[idx].deriv = None;
        }
        let shape = self[node].value.as_ref().unwrap().shape();
        let one = Tensor::ones(shape);
        self.deriv_recur(node, one)
    }

    pub fn to_dot(&self, sink: &mut impl io::Write) -> io::Result<()>
    where
        A: fmt::Debug,
    {
        use petgraph::dot;
        let dot = dot::Dot::with_config(&self.graph, &[dot::Config::EdgeNoLabel]);
        write!(sink, "{:?}", dot)?;
        Ok(())
    }
}
