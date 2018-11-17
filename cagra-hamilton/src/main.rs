#[macro_use]
extern crate cagra;

use failure::Error;

fn main() -> Result<(), Error> {
    let mut g = graph!(f64, {
        let q = 1.0;
        let p = 1.0;
        let h = square(q) + square(p);
    });
    let q = g.get_index("q");
    let p = g.get_index("p");
    let h = g.get_index("h");

    let dt = 0.01;
    println!("E,q,p"); // csv header
    for _ in 0..200 {
        let e = g.eval_value(h)?;
        g.eval_deriv(h)?;

        let vp = g.get_value(p)?;
        let hq = g.get_deriv(q)?;

        g.set_value(p, vp - dt * hq)?;

        g.eval_value(h)?;
        g.eval_deriv(h)?;

        let vq = g.get_value(q)?;
        let hp = g.get_deriv(p)?;
        g.set_value(q, vq + dt * hp)?;

        println!("{:.09},{:.09},{:.09}", e, vq, vp);
    }
    Ok(())
}
