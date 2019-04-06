use cagra::graph::*;
use criterion::{criterion_group, criterion_main, Criterion};

// x = x + 1 + 1 + ...
fn linear(c: &mut Criterion) {
    c.bench_function("eval_value", |b| {
        let mut g: Graph<f64> = Graph::new();
        let mut x = g.scalar("x", 0.0).unwrap();
        for _ in 0..1000 {
            let v = g.constant_scalar(1.0);
            x = g.add(x, v);
        }
        b.iter(|| g.eval_value(x))
    });

    c.bench_function("eval_deriv", |b| {
        let mut g: Graph<f64> = Graph::new();
        let mut x = g.scalar("x", 0.0).unwrap();
        for _ in 0..1000 {
            let v = g.constant_scalar(1.0);
            x = g.add(x, v);
        }
        g.eval_value(x).unwrap();
        b.iter(|| g.eval_deriv(x))
    });
}

criterion_group!(benches, linear);
criterion_main!(benches);
