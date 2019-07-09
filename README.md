# rust-postal
> Bindings to the libpostal street address parsing/normalization C library.

[![Build Status](https://travis-ci.com/pnordahl/rust-postal.svg?branch=master)](https://travis-ci.com/pnordahl/rust-postal)
[![Cargo](https://img.shields.io/crates/v/postal.svg)](https://crates.io/crates/postal)

This library provides [rust-lang/rust-bindgen](https://github.com/rust-lang/rust-bindgen) generated Rust -> C bindings, and puts an ergonomic and safe Rust API on top of them.

Still TODO:
- [ ] normalize_address support
- [ ] CI

## Installation

Follow the README instructions at [openvenues/libpostal](https://github.com/openvenues/libpostal) to install the shared library for your platform. Currently, the compiled object is dynamically linked when your project runs - static linkage could be supported in the future.

Add `postal` to your `Cargo.toml`:

Add this to your Cargo.toml:
```
[dependencies]
postal = "0.1"
```

Next, add this to your crate:
```
extern crate postal;
```

## Usage example (expand_address)

*Note*: `libpostal` is not threadsafe. As a result, do not create more than one `postal::Context` per process. `Context::expand_address` and `Context::normalize_address` do internal locking, and are safe to call concurrently.

This is a minimal example of using the `expand_address` API:

```rust
extern crate postal;
use postal::{Context, InitOptions, ExpandAddressOptions};

// initialize a context to work with
let mut ctx = Context::new();

// enable address expansion for this context
ctx.init(InitOptions{expand_address: true}).unwrap();

// these options are safe to persist and reuse between calls to `expand_address`
let mut opts = ExpandAddressOptions::new();

// (optional) set languages; this can improve runtime performance significantly, approximately 30% in benchmarks
opts.set_languages(vec!["en"].as_slice());

// expand a single address into a `postal::Expansions` iterator
let exps = ctx.expand_address(
	"1234 Cherry Ln, Podunk TX", &mut opts)
	.unwrap();
for e in exps {
	dbg!(e);
}
```

_For more examples and usage, please refer to the tests or benchmarks._

## Development setup

This will build `bindgen` bindings, run the tests, and run the benchmarks.

```sh
cargo build
cargo test -- --nocapture --test-threads 1
cargo bench
```

Note: `--test-threads 1` is required due to the single-threaded nature of `libpostal`.

## Release History

* 0.1.0
    * Initial release

## Meta

Distributed under the MIT license. See ``LICENSE`` for more information.

Copyright [Knockway Inc.](https://www.knock.com) and its affiliates.


## Contributing

1. Fork it (<https://github.com/pnordahl/rust-postal/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
