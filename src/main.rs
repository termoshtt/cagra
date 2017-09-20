
#[macro_use]
extern crate procedurals;
extern crate petgraph;
extern crate ndarray;
extern crate ndarray_linalg;

pub mod graph;

use graph::Graph;

fn main() {
    let mut g: Graph<f64> = Graph::new();
    let x = g.variable("x");
    let y = g.variable("y");
    let x_y = g.plus(x, y);
    let z = g.variable("z");
    let _ = g.plus(x_y, z);
    println!("{:?}", g);
}
