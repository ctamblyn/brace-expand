use brace_expand::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn simple_expansion(c: &mut Criterion) {
    let input = "aaa{bbb,ccc}ddd";

    c.bench_function("simple expansion", |b| {
        b.iter(|| brace_expand(black_box(&input)))
    });
}

fn nested_expansion(c: &mut Criterion) {
    let input = "{aaa,bbb}ccc{eee,fff{ggg,hhh}}";

    c.bench_function("nested expansion", |b| {
        b.iter(|| brace_expand(black_box(&input)))
    });
}

criterion_group!(benches, simple_expansion, nested_expansion);
criterion_main!(benches);
