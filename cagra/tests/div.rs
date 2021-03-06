use approx::abs_diff_eq;
use cagra::{error::Result, graph::Graph, tensor::*};

#[test]
fn test_div() -> Result<()> {
    let mut g = Graph::new();
    let x = g.scalar("x", 1.0)?;
    let y = g.scalar("y", 2.0)?;
    let z = g.div(x, y);
    abs_diff_eq!(g.eval_value(z)?.as_scalar()?, 0.5);
    g.eval_deriv(z)?;
    abs_diff_eq!(g.get_deriv(x)?.as_scalar()?, 0.5);
    abs_diff_eq!(g.get_deriv(y)?.as_scalar()?, -0.25);
    Ok(())
}
