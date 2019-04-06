use approx::abs_diff_eq;
use cagra::{error::Result, graph::Graph, tensor::*};

#[test]
fn test_square() -> Result<()> {
    let mut g = Graph::new();
    let x = g.scalar("x", 3.0)?;
    let y = g.square(x);
    abs_diff_eq!(g.eval_value(y)?.as_scalar()?, 9.0);
    g.eval_deriv(y)?;
    abs_diff_eq!(g.get_deriv(x)?.as_scalar()?, 6.0);
    Ok(())
}
