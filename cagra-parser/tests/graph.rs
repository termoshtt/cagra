use cagra_parser::graph_impl;

graph_impl!(f64, {
    let x = 1.2;
    let y = 2.0 * sin(x);
    let z = x + y + 2.0 * x * y;
});
