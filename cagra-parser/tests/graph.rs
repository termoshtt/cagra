use cagra_parser::graph_impl;

graph_impl!({
    let x = 1.2;
    let y = sin(x);
    let z = x + y;
});
