use approx::abs_diff_eq;
use cagra::{error::Result, graph::Graph};

#[test]
fn test_pow2() -> Result<()> {
    let mut g = Graph::new();
    let x = g.variable("x", 3.0)?;
    let y = g.pow2(x);
    abs_diff_eq!(g.eval_value(y)?, 9.0);
    g.eval_deriv(y)?;
    abs_diff_eq!(g.get_deriv(x)?, 6.0);
    Ok(())
}

