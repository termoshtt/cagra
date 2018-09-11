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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node<A: Field> {
    value: Option<A>,
    deriv: Option<A>,
    prop: Property,
}

/// Extra propaties of the `Node` accoding to the node type.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum Property {
    Variable,
    Unary(Unary),
    Binary(Binary),
}

impl<A: Field> Node<A> {
    /// Check the node is variable
    pub fn is_variable(&self) -> bool {
        match self.prop {
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
    name_space: HashMap<String, NodeIndex>,
}

impl<A: Field> Graph<A> {
    /// new graph.
    pub fn new() -> Self {
        Self {
            graph: petgraph::graph::Graph::new(),
            name_space: HashMap::new(),
        }
    }

    /// Create new empty variable
    pub fn empty_variable(&mut self, name: &str) -> Result<NodeIndex> {
        // check name duplication
        match self.name_space.entry(name.into()) {
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

    pub fn get_index(&self, name: &str) -> Result<NodeIndex> {
        self.name_space
            .get(name)
            .cloned()
            .ok_or(Error::UndefinedName { name: name.into() })
    }

    pub fn set_index(&mut self, name: &str, id: NodeIndex) -> Result<()> {
        // check name duplication
        match self.name_space.entry(name.into()) {
            Entry::Occupied(_) => Err(Error::DuplicatedName { name: name.into() }),
            Entry::Vacant(entry) => {
                entry.insert(id);
                Ok(())
            }
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

    /// Get the value of the node. If the value has not been caluclated,
    /// returns `None`
    pub fn get_value(&self, node: NodeIndex) -> Option<A> {
        self.graph[node].value
    }

    /// Get the value of the node. If the value has not been caluclated,
    /// returns `None`
    pub fn get_deriv(&self, node: NodeIndex) -> Option<A> {
        self.graph[node].deriv
    }

    /// Evaluate the value of the node recusively.
    ///
    /// * `use_cached` - Use the value if already calculated.
    pub fn eval_value(&mut self, node: NodeIndex, use_cached: bool) {
        // FIXME This code traces the graph twice, but it may be able to done by once.
        let prop = self.graph[node].prop.clone();
        let value_exists = self.graph[node].value.is_some();
        match prop {
            Property::Variable => {
                if value_exists {
                    return;
                }
            }
            Property::Unary(ref op) => {
                if use_cached && value_exists {
                    return;
                }
                let arg = self.get_arg1(node);
                self.eval_value(arg, use_cached);
                self.graph[node].value = Some(op.eval_value(self.get_value(arg).unwrap()));
            }
            Property::Binary(ref op) => {
                if use_cached && value_exists {
                    return;
                }
                let (lhs, rhs) = self.get_arg2(node);
                self.eval_value(rhs, use_cached);
                self.eval_value(lhs, use_cached);
                let res = op.eval_value(self.get_value(lhs).unwrap(), self.get_value(rhs).unwrap());
                self.graph[node].value = Some(res);
            }
        };
    }

    fn deriv_recur(&mut self, node: NodeIndex, der: A) {
        self.graph[node].deriv = Some(der);
        let prop = self.graph[node].prop.clone();
        match prop {
            Property::Variable => {}
            Property::Unary(ref op) => {
                let arg = self.get_arg1(node);
                let der =
                    op.eval_deriv(self.get_value(arg).unwrap(), self.get_deriv(node).unwrap());
                self.deriv_recur(arg, der);
            }
            Property::Binary(ref op) => {
                let (lhs, rhs) = self.get_arg2(node);
                let (l_der, r_der) = op.eval_deriv(
                    self.get_value(lhs).unwrap(),
                    self.get_value(rhs).unwrap(),
                    self.get_deriv(node).unwrap(),
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
