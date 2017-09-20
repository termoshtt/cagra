
#[macro_use]
extern crate procedurals;
extern crate petgraph;
extern crate ndarray;
extern crate ndarray_linalg;

pub mod graph;

use graph::Graph;

fn main() {
    let mut g: Graph<f64> = Graph::new();
    let v1 = g.variable();
    let v2 = g.variable();
    let v1p2 = g.plus(v1, v2);
    let v3 = g.variable();
    let _ = g.plus(v1p2, v3);
    println!("{:?}", g);
}
