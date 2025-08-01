use criterion::{criterion_group, criterion_main, Criterion};
use nprint_core::{expand_macro, MacroDef, MacroElem};

fn bench_expand(c: &mut Criterion) {
    let def = MacroDef {
        name: "test".to_string(),
        param_count: 1,
        template: vec![MacroElem::Param(0), MacroElem::Op(OP_DUP)],
    };
    c.bench_function("expand_macro", |b| b.iter(|| expand_macro(&def, &[5])));
}

criterion_group!(benches, bench_expand);
criterion_main!(benches);
