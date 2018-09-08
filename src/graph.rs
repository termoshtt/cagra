//! Calculation graph

use petgraph;
use petgraph::prelude::*;

use error::*;
use operators::*;
use scalar::Field;

/// Node of the calculation graph.
///
/// This struct keeps the last value, and `Graph` calculates the derivative
/// using this value.
#[derive(Debug, Clone)]
pub struct Node<A: Field> {
    value: Option<A>,
    deriv: Option<A>,
    prop: Property,
}

/// Extra propaties of the `Node` accoding to the node type.
#[derive(Debug, Clone, IntoEnum)]
enum Property {
    Variable(Variable),
    UnaryOperator(UnaryOperatorAny),
    BinaryOperator(BinaryOperatorAny),
}

impl<A: Field> Node<A> {
    fn new(prop: Property) -> Self {
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
            Property::UnaryOperator(_) => false,
            Property::BinaryOperator(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
}

impl Variable {
    fn new(name: &str) -> Self {
        Variable {
            name: name.to_string(),
        }
    }
}

/// Calculation graph based on `petgraph::graph::Graph`
#[derive(Debug, NewType)]
pub struct Graph<A: Field>(petgraph::graph::Graph<Node<A>, ()>);

impl<A: Field> Graph<A> {
    /// new graph.
    pub fn new() -> Self {
        petgraph::graph::Graph::new().into()
    }

    /// Create new empty variable
    pub fn empty_variable(&mut self, name: &str) -> NodeIndex {
        let var = Variable::new(name);
        self.add_node(Node::new(var.into()))
    }

    /// Create new variable with value
    pub fn variable(&mut self, name: &str, value: A) -> NodeIndex {
        let var = self.empty_variable(name);
        self.set_value(var, value).unwrap();
        var
    }

    /// Set a value to a variable node, and returns `NodeTypeError` if the node is an operator.
    pub fn set_value(&mut self, node: NodeIndex, value: A) -> Result<()> {
        if self[node].is_variable() {
            self[node].value = Some(value.into());
            Ok(())
        } else {
            Err(Error::NodeTypeError {})
        }
    }

    pub fn add(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let p = self.add_node(Node::new(add().into()));
        self.add_edge(lhs, p, ());
        self.add_edge(rhs, p, ());
        p
    }

    pub fn mul(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let p = self.add_node(Node::new(mul().into()));
        self.add_edge(lhs, p, ());
        self.add_edge(rhs, p, ());
        p
    }

    pub fn neg(&mut self, arg: NodeIndex) -> NodeIndex {
        let n = self.add_node(Node::new(neg().into()));
        self.add_edge(arg, n, ());
        n
    }

    pub fn sub(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        let m_rhs = self.neg(rhs);
        self.add(lhs, m_rhs)
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
            Property::UnaryOperator(ref op) => {
                if use_cached && value_exists {
                    return;
                }
                let arg = self.get_arg1(node);
                self.eval_value(arg, use_cached);
                self[node].value = Some(op.eval_value(self.get_value(arg).unwrap()));
            }
            Property::BinaryOperator(ref op) => {
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
            Property::UnaryOperator(ref op) => {
                let arg = self.get_arg1(node);
                let der =
                    op.eval_deriv(self.get_value(arg).unwrap(), self.get_deriv(node).unwrap());
                self.deriv_recur(arg, der);
            }
            Property::BinaryOperator(ref op) => {
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
