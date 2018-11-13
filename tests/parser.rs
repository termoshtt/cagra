#[macro_use]
extern crate cagra;

#[test]
fn test_graph_macro() {
    let _g = graph!(f64, {
        let x = 1.0;
        let y = 3.0;
        let z = x + y + 2.0 * x * y;
    });
}
