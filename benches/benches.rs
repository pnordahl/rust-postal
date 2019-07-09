// Knockway Inc. and its affiliates. All Rights Reserved

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use postal::{Context, ExpandAddressOptions, InitOptions};

fn criterion_benchmark(c: &mut Criterion) {
    let mut ctx = Context::new();
    ctx.init(InitOptions {
        expand_address: true,
    })
    .unwrap();

    let mut opts = black_box(ExpandAddressOptions::new());
    opts.set_languages(vec!["en"].as_slice());

    c.bench_function("expand_address address 1", move |b| {
        let addr = black_box("1234 Cherry Ln Podunk TX");
        b.iter(|| {
            ctx.expand_address(addr, &mut opts).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
