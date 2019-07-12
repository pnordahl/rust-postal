// Knockway Inc. and its affiliates. All Rights Reserved

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use postal::{Context, ExpandAddressOptions, InitOptions, ParseAddressOptions};

fn criterion_benchmark(c: &mut Criterion) {
    // expand_address
    let mut ctx = Context::new();
    ctx.init(InitOptions {
        expand_address: true,
        parse_address: false,
    })
    .unwrap();

    let mut opts = ExpandAddressOptions::new();
    opts.set_languages(vec!["en"].as_slice());
    let mut bbopts = black_box(opts);

    c.bench_function("expand_address address 1", move |b| {
        let addr = black_box("1234 Cherry Ln Podunk TX");
        b.iter(|| {
            ctx.expand_address(addr, &mut bbopts).unwrap();
        })
    });

    // parse_address
    println!("loading address parser...");
    let mut ctx2 = Context::new();
    ctx2.init(InitOptions {
        expand_address: false,
        parse_address: true,
    })
    .unwrap();
    println!("done");

    let opts2 = ParseAddressOptions::new();
    let mut bbopts2 = black_box(opts2);
    c.bench_function("parse_address address 1", move |b| {
        let addr = black_box("1234 Cherry Ln Podunk TX");
        b.iter(|| {
            ctx2.parse_address(addr, &mut bbopts2).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
