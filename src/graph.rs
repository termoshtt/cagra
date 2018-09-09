//! Calculation graph

use petgraph;
use petgraph::prelude::*;
use std::collections::{hash_map::Entry, HashMap};
use std::marker::PhantomData;

use super::error::*;
use super::operators::*;
use super::scalar::Field;

/// Node of the calculation graph.
///
/// This struct keeps the last value, and `Graph` calculates the derivative
/// using this value.
#[derive(Debug)]
pub struct Node<A: Field> {
    value: Option<A>,
    deriv: Option<A>,
    prop: Property<A>,
}

/// Extra propaties of the `Node` accoding to the node type.
#[derive(Debug, Clone)]
enum Property<A> {
    Variable(Variable<A>),
    UnaryOp(UnaryOp<A>),
    BinOp(BinOp<A>),
}

impl<A: Field> Node<A> {
    fn variable(var: Variable<A>) -> Self {
        Self::new(Property::Variable(var))
    }

    fn unray_op(op: UnaryOp<A>) -> Self {
        Self::new(Property::UnaryOp(op))
    }

    fn bin_op(op: BinOp<A>) -> Self {
        Self::new(Property::BinOp(op))
    }

    fn new(prop: Property<A>) -> Self {
        Self {
            value: None,
            deriv: None,
            prop,
        }
    }

    /// Check the node is variable
    pub fn is_variable(&self) -> bool {
        match self.prop {
            Property::Variable(_) => true,
            Property::UnaryOp(_) => false,
            Property::BinOp(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Variable<A> {
    name: String,
    phantom: PhantomData<A>,
}

impl<A> Variable<A> {
    fn new(name: &str) -> Self {
        Variable {
            name: name.to_string(),
            phantom: PhantomData,
        }
    }
}

/// Calculation graph based on `petgraph::graph::Graph`
#[derive(Debug)]
pub struct Graph<A: Field> {
    graph: petgraph::graph::Graph<Node<A>, ()>,
    name_space: HashMap<String, NodeIndex>,
}

impl<A: Field> ::std::ops::Deref for Graph<A> {
    type Target = petgraph::graph::Graph<Node<A>, ()>;
    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<A: Field> ::std::ops::DerefMut for Graph<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
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
                let var = Variable::new(name);
                let id = self.graph.add_node(Node::variable(var));
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
        if self[node].is_variable() {
            self[node].value = Some(value.into());
            Ok(())
        } else {
            Err(Error::NodeTypeError {
                index: node.index(),
            })
        }
    }

    pub fn add(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let p = self.add_node(Node::bin_op(add()));
        self.add_edge(lhs, p, ());
        self.add_edge(rhs, p, ());
        p
    }

    pub fn mul(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let p = self.add_node(Node::bin_op(mul()));
        self.add_edge(lhs, p, ());
        self.add_edge(rhs, p, ());
        p
    }

    pub fn neg(&mut self, arg: NodeIndex) -> NodeIndex {
        let n = self.add_node(Node::unray_op(neg()));
        self.add_edge(arg, n, ());
        n
    }

    pub fn sub(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let m_rhs = self.neg(rhs);
        self.add(lhs, m_rhs)
    }

    pub fn get_index(&self, name: &str) -> Result<NodeIndex> {
        self.name_space.get(name).cloned().ok_or(Error::UndefinedName { name: name.into() })
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
    pub fn get_value(&self, node: NodeIndex) -> Option<A> {
        self[node].value
    }

    /// Get the value of the node. If the value has not been caluclated,
    /// returns `None`
    pub fn get_deriv(&self, node: NodeIndex) -> Option<A> {
        self[node].deriv
    }

    /// Evaluate the value of the node recusively.
    ///
    /// * `use_cached` - Use the value if already calculated.
    pub fn eval_value(&mut self, node: NodeIndex, use_cached: bool) {
        // FIXME This code traces the graph twice, but it may be able to done by once.
        let prop = self[node].prop.clone();
        let value_exists = self[node].value.is_some();
        match prop {
            Property::Variable(ref v) => {
                if value_exists {
                    return;
                }
                panic!("Variable '{}' is evaluated before set value", v.name)
            }
            Property::UnaryOp(ref op) => {
                if use_cached && value_exists {
                    return;
                }
                let arg = self.get_arg1(node);
                self.eval_value(arg, use_cached);
                self[node].value = Some(op.eval_value(self.get_value(arg).unwrap()));
            }
            Property::BinOp(ref op) => {
                if use_cached && value_exists {
                    return;
                }
                let (lhs, rhs) = self.get_arg2(node);
                self.eval_value(rhs, use_cached);
                self.eval_value(lhs, use_cached);
                let res = op.eval_value(self.get_value(lhs).unwrap(), self.get_value(rhs).unwrap());
                self[node].value = Some(res);
            }
        };
    }

    fn deriv_recur(&mut self, node: NodeIndex, der: A) {
        self[node].deriv = Some(der);
        let prop = self[node].prop.clone();
        match prop {
            Property::Variable(_) => {}
            Property::UnaryOp(ref op) => {
                let arg = self.get_arg1(node);
                let der =
                    op.eval_deriv(self.get_value(arg).unwrap(), self.get_deriv(node).unwrap());
                self.deriv_recur(arg, der);
            }
            Property::BinOp(ref op) => {
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
}
