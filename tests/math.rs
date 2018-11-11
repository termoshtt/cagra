use approx::abs_diff_eq;
use cagra::{error::Result, graph::Graph};

#[test]
fn test_exp() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let y = g.exp(x);
    abs_diff_eq!(g.eval_value(y)?, x0.exp());
    g.eval_deriv(y)?;
    abs_diff_eq!(g.get_deriv(x)?, x0.exp());
    Ok(())
}

#[test]
fn test_ln() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let y = g.ln(x);
    abs_diff_eq!(g.eval_value(y)?, x0.ln());
    g.eval_deriv(y)?;
    abs_diff_eq!(g.get_deriv(x)?, 1.0 / x0);
    Ok(())
}

#[test]
fn test_ln_exp() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let y = g.exp(x);
    let z = g.exp(y);
    abs_diff_eq!(g.eval_value(z)?, x0);
    g.eval_deriv(z)?;
    abs_diff_eq!(g.get_deriv(x)?, 0.0);
    Ok(())
}

#[test]
fn test_sin() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let y = g.sin(x);
    abs_diff_eq!(g.eval_value(y)?, x0.sin());
    g.eval_deriv(y)?;
    abs_diff_eq!(g.get_deriv(x)?, x0.cos());
    Ok(())
}

#[test]
fn test_cos() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let y = g.cos(x);
    abs_diff_eq!(g.eval_value(y)?, x0.cos());
    g.eval_deriv(y)?;
    abs_diff_eq!(g.get_deriv(x)?, -x0.sin());
    Ok(())
}

#[test]
fn test_sin_cos() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let s = g.sin(x);
    let ss = g.square(s);
    let c = g.cos(x);
    let cc = g.square(c);
    let z = g.add(ss, cc);
    abs_diff_eq!(g.eval_value(z)?, 1.0);
    g.eval_deriv(z)?;
    abs_diff_eq!(g.get_deriv(x)?, 0.0);
    Ok(())
}

#[test]
fn test_tan() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let t = g.tan(x);
    let s = g.sin(x);
    let c = g.cos(x);
    let tc = g.mul(t, c);
    let z = g.div(tc, s);
    abs_diff_eq!(g.eval_value(z)?, 1.0);
    g.eval_deriv(z)?;
    abs_diff_eq!(g.get_deriv(x)?, 0.0);
    Ok(())
}

#[test]
fn test_sinh() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let y = g.sinh(x);
    abs_diff_eq!(g.eval_value(y)?, x0.sinh());
    g.eval_deriv(y)?;
    abs_diff_eq!(g.get_deriv(x)?, x0.cosh());
    Ok(())
}

#[test]
fn test_cosh() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let y = g.cosh(x);
    abs_diff_eq!(g.eval_value(y)?, x0.cosh());
    g.eval_deriv(y)?;
    abs_diff_eq!(g.get_deriv(x)?, x0.sinh());
    Ok(())
}

#[test]
fn test_sinh_cosh() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let s = g.sinh(x);
    let ss = g.square(s);
    let c = g.cosh(x);
    let cc = g.square(c);
    let z = g.sub(cc, ss);
    abs_diff_eq!(g.eval_value(z)?, 1.0);
    g.eval_deriv(z)?;
    abs_diff_eq!(g.get_deriv(x)?, 0.0);
    Ok(())
}

#[test]
fn test_tanh() -> Result<()> {
    let mut g = Graph::new();
    let x0: f32 = 1.234;
    let x = g.variable("x", x0)?;
    let t = g.tanh(x);
    let s = g.sinh(x);
    let c = g.cosh(x);
    let tc = g.mul(t, c);
    let z = g.div(tc, s);
    abs_diff_eq!(g.eval_value(z)?, 1.0);
    g.eval_deriv(z)?;
    abs_diff_eq!(g.get_deriv(x)?, 0.0);
    Ok(())
}
