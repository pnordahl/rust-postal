// Knockway Inc. and its affiliates. All Rights Reserved

#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use postal::{Context, ExpandAddressOptions, InitOptions, ParseAddressOptions};
use std::rc::Rc;

fn criterion_benchmark(c: &mut Criterion) {
    println!("Loading context with all initialization options...");
    let mut ctx = Context::new();
    ctx.init(InitOptions {
        expand_address: true,
        parse_address: true,
    })
    .unwrap();
    let ctx_rc = Rc::new(ctx);
    println!("Done, running benchmarks...");

    // expand_address
    let ctx2 = Rc::clone(&ctx_rc);
    let mut opts = ExpandAddressOptions::new();
    opts.set_languages(vec!["en"].as_slice());
    let mut bbopts = black_box(opts);
    c.bench_function("expand_address address 1", move |b| {
        let addr = black_box("1234 Cherry Ln Podunk TX");
        b.iter(|| {
            ctx2.expand_address(addr, &mut bbopts).unwrap();
        })
    });

    // parse_address
    let ctx3 = Rc::clone(&ctx_rc);
    let opts2 = ParseAddressOptions::new();
    let mut bbopts2 = black_box(opts2);
    c.bench_function("parse_address, default options", move |b| {
        let addr = black_box("1234 Cherry Ln Podunk TX");
        b.iter(|| {
            ctx3.parse_address(addr, &mut bbopts2).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
