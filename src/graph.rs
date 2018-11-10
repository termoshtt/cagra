//! Calculation graph

use petgraph::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::{hash_map::Entry, HashMap};
use std::io;

use super::error::{Error, Result};
use super::operator::{Binary, Unary};
use super::scalar::Field;

/// Node of the calculation graph.
///
/// This struct keeps the last value, and `Graph` calculates the derivative
/// using this value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node<A: Field> {
    value: Option<A>,
    deriv: Option<A>,
    property: Property,
}

/// Extra propaties of the `Node` accoding to the node type.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
enum Property {
    Constant,
    Variable,
    Unary(Unary),
    Binary(Binary),
}

impl<A: Field> Node<A> {
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

    fn constant(a: A) -> Self {
        Self {
            value: Some(a),
            deriv: None,
            property: Property::Constant,
        }
    }
}

impl<A: Field> From<Unary> for Node<A> {
    fn from(op: Unary) -> Self {
        Self {
            value: None,
            deriv: None,
            property: Property::Unary(op),
        }
    }
}

impl<A: Field> From<Binary> for Node<A> {
    fn from(op: Binary) -> Self {
        Self {
            value: None,
            deriv: None,
            property: Property::Binary(op),
        }
    }
}

/// Calculation graph based on `petgraph::graph::Graph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph<A: Field> {
    graph: petgraph::graph::Graph<Node<A>, ()>,
    namespace: HashMap<String, NodeIndex>,
}

// Panic if the index does not exists
impl<A: Field> ::std::ops::Index<NodeIndex> for Graph<A> {
    type Output = Node<A>;
    fn index(&self, index: NodeIndex) -> &Node<A> {
        &self.graph[index]
    }
}

// Panic if the index does not exists
impl<A: Field> ::std::ops::IndexMut<NodeIndex> for Graph<A> {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Node<A> {
        &mut self.graph[index]
    }
}

// Panic if the name is not found
impl<A: Field> ::std::ops::Index<&str> for Graph<A> {
    type Output = Node<A>;
    fn index(&self, name: &str) -> &Node<A> {
        let index = self.namespace[name];
        &self.graph[index]
    }
}

// Panic if the name is not found
impl<A: Field> ::std::ops::IndexMut<&str> for Graph<A> {
    fn index_mut(&mut self, name: &str) -> &mut Node<A> {
        let index = self.namespace[name];
        &mut self.graph[index]
    }
}

impl<A: Field> Graph<A> {
    /// new graph.
    pub fn new() -> Self {
        Self {
            graph: petgraph::graph::Graph::new(),
            namespace: HashMap::new(),
        }
    }

    pub fn constant(&mut self, value: A) -> NodeIndex {
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

    /// Create new variable with value
    pub fn variable(&mut self, name: &str, value: A) -> Result<NodeIndex> {
        let var = self.empty_variable(name)?;
        self.set_value(var, value).unwrap();
        Ok(var)
    }

    /// Set a value to a variable node, and returns `NodeTypeError` if the node is an operator.
    pub fn set_value(&mut self, node: NodeIndex, value: A) -> Result<()> {
        if self.graph[node].is_variable() {
            self.graph[node].value = Some(value.into());
            Ok(())
        } else {
            Err(Error::NodeTypeError {
                index: node.index(),
            })
        }
    }

    pub fn add(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let p = self.graph.add_node(Binary::Add.into());
        self.graph.add_edge(lhs, p, ());
        self.graph.add_edge(rhs, p, ());
        p
    }

    pub fn mul(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let p = self.graph.add_node(Binary::Mul.into());
        self.graph.add_edge(lhs, p, ());
        self.graph.add_edge(rhs, p, ());
        p
    }

    pub fn neg(&mut self, arg: NodeIndex) -> NodeIndex {
        let n = self.graph.add_node(Unary::Neg.into());
        self.graph.add_edge(arg, n, ());
        n
    }

    pub fn sub(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let m_rhs = self.neg(rhs);
        self.add(lhs, m_rhs)
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
    pub fn eval_value(&mut self, node: NodeIndex) -> Result<A> {
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
                self[node].value = Some(value); // cache
                value
            })),
            Property::Binary(ref op) => Ok(self.get_value(node).unwrap_or({
                let (lhs, rhs) = self.get_arg2(node);
                let lv = self.eval_value(lhs)?;
                let rv = self.eval_value(rhs)?;
                let value = op.eval_value(lv, rv);
                self[node].value = Some(value); // cache
                value
            })),
        }
    }

    pub fn get_value(&self, node: NodeIndex) -> Result<A> {
        self[node].value.ok_or(Error::ValueUninitialized {
            index: node.index(),
        })
    }

    pub fn get_deriv(&self, node: NodeIndex) -> Result<A> {
        self[node].deriv.ok_or(Error::DerivUninitialized {
            index: node.index(),
        })
    }

    fn deriv_recur(&mut self, node: NodeIndex, der: A) -> Result<()> {
        self[node].deriv = match self[node].deriv {
            Some(der_last) => Some(der_last + der),
            None => Some(der),
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
        self.deriv_recur(node, A::one())
    }

    pub fn to_json(&self, writer: impl io::Write) -> Result<()> {
        serde_json::to_writer(writer, self).map_err(|error| Error::JSONSerializeFailed { error })
    }

    pub fn to_json_str(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|error| Error::JSONSerializeFailed { error })
    }
}
