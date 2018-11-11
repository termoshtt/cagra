use cagra_parser::graph_impl;

graph_impl!({
    let x = 1;
    let y = sin(x);
    let z = x + y;
});

#[test]
fn test_graph_new() {
    let g = graph_new();
}
