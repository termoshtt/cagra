//! Calculation graph

use petgraph::prelude::*;
use std::collections::{hash_map::Entry, HashMap};
use std::io;

use super::error::{Error, Result};
use super::operator::{Binary, Unary};
use super::scalar::Field;

/// Node of the calculation graph.
///
/// This struct keeps the last value, and `Graph` calculates the derivative
/// using this value.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Node<A: Field> {
    value: Option<A>,
    deriv: Option<A>,
    prop: Property,
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
        match self.prop {
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
            prop: Property::Variable,
        }
    }

    fn constant(a: A) -> Self {
        Self {
            value: Some(a),
            deriv: None,
            prop: Property::Constant,
        }
    }
}

impl<A: Field> From<Unary> for Node<A> {
    fn from(op: Unary) -> Self {
        Self {
            value: None,
            deriv: None,
            prop: Property::Unary(op),
        }
    }
}

impl<A: Field> From<Binary> for Node<A> {
    fn from(op: Binary) -> Self {
        Self {
            value: None,
            deriv: None,
            prop: Property::Binary(op),
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
        let mut n = self.graph[node];
        match n.prop {
            Property::Variable => n.value.ok_or(Error::Uninitialized {
                index: node.index(),
            }),
            Property::Constant => Ok(n.value.expect("Constant is not initialized")),
            Property::Unary(ref op) => Ok(n.value.unwrap_or({
                let arg = self.get_arg1(node);
                let val1 = self.eval_value(arg)?;
                let value = op.eval_value(val1);
                n.value = Some(value); // cache
                value
            })),
            Property::Binary(ref op) => Ok(n.value.unwrap_or({
                let (lhs, rhs) = self.get_arg2(node);
                let lv = self.eval_value(lhs)?;
                let rv = self.eval_value(rhs)?;
                let value = op.eval_value(lv, rv);
                n.value = Some(value); // cache
                value
            })),
        }
    }

    fn deriv_recur(&mut self, node: NodeIndex, der: A) {
        self.graph[node].deriv = Some(der);
        let prop = self.graph[node].prop;
        match prop {
            Property::Variable | Property::Constant => {}
            Property::Unary(ref op) => {
                let arg = self.get_arg1(node);
                let der = op.eval_deriv(self[arg].value.unwrap(), self[node].deriv.unwrap());
                self.deriv_recur(arg, der);
            }
            Property::Binary(ref op) => {
                let (lhs, rhs) = self.get_arg2(node);
                let (l_der, r_der) = op.eval_deriv(
                    self[lhs].value.unwrap(),
                    self[rhs].value.unwrap(),
                    self[node].deriv.unwrap(),
                );
                self.deriv_recur(lhs, l_der);
                self.deriv_recur(rhs, r_der);
            }
        };
    }

    /// Evaluate derivative recursively.
    pub fn eval_deriv(&mut self, node: NodeIndex) {
        self.deriv_recur(node, A::one())
    }

    pub fn to_json(&self, writer: impl io::Write) -> Result<()> {
        serde_json::to_writer(writer, self).map_err(|error| Error::JSONSerializeFailed { error })
    }

    pub fn to_json_str(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|error| Error::JSONSerializeFailed { error })
    }
}
