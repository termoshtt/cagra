use approx::abs_diff_eq;
use cagra::{error::Result, graph::Graph, tensor::*};

#[test]
fn test_dot() -> Result<()> {
    let mut g = Graph::new();
    let x0: &[f64] = &[1.0, 2.0];
    let y0: &[f64] = &[3.0, 4.0];
    let x = g.vector("x", x0)?;
    let y = g.vector("y", y0)?;
    let z = g.dot(x, y);
    abs_diff_eq!(g.eval_value(z)?.as_scalar()?, 1.0 * 3.0 + 2.0 * 4.0);
    g.eval_deriv(z)?;
    abs_diff_eq!(g.get_deriv(x)?.as_vector()?, y0);
    abs_diff_eq!(g.get_deriv(y)?.as_vector()?, x0);
    Ok(())
}
