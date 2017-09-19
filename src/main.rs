
extern crate petgraph;

use std::ops::Add;
use std::fmt::Debug;

pub type Ix = petgraph::graph::DefaultIx;

#[derive(Debug)]
pub enum Node<Value>
where
    Value: Add<Output = Value> + Debug,
{
    Variable(Value),
    Plus(Plus),
    End(()),
}

#[derive(Debug)]
pub struct Edge;

#[derive(Debug)]
pub struct Plus;

pub type Graph<Value> = petgraph::graph::Graph<Node<Value>, Edge, petgraph::Directed, Ix>;

fn main() {
    let mut g = Graph::new();
    let e = g.add_node(Node::End(()));
    let p = g.add_node(Node::Plus(Plus {}));
    let v1 = g.add_node(Node::Variable(1));
    let v2 = g.add_node(Node::Variable(2));
    g.add_edge(p, e, Edge {});
    g.add_edge(v1, p, Edge {});
    g.add_edge(v2, p, Edge {});

    println!("{:?}", g);
}
