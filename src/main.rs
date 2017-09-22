
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate procedurals;
extern crate petgraph;
extern crate ndarray;
extern crate ndarray_linalg;

pub mod graph;
pub mod error;

use graph::Graph;

fn main() {
    let mut g: Graph<f64> = Graph::new();
    let x = g.variable("x");
    g.set_value(x, 1.0);
    let y = g.variable("y");
    g.set_value(y, 2.0);
    let x_y = g.plus(x, y);
    let z = g.variable("z");
    g.set_value(z, 3.0);
    let sum = g.plus(x_y, z);
    g.eval(sum, false).unwrap();
    println!("{:?}", g);
}
