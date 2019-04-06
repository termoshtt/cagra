use cagra::graph;
use std::fs;

fn main() -> Result<(), failure::Error> {
    let mut g = graph!(f64, {
        let x = 1.0;
        let y = x * 2.0;
        let z = square(y);
    });
    g.to_dot(&mut fs::File::create("init.dot")?)?;

    let z = g.get_index("z");
    g.eval_value(z)?;
    g.to_dot(&mut fs::File::create("eval_value.dot")?)?;

    g.eval_deriv(z)?;
    g.to_dot(&mut fs::File::create("eval_deriv.dot")?)?;
    Ok(())
}
