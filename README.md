# rust-postal
> Bindings to the libpostal street address parsing/normalization C library.

![Build Status](https://github.com/github/docs/actions/workflows/main.yml/badge.svg?branch=master)

[![Cargo](https://img.shields.io/crates/v/postal.svg)](https://crates.io/crates/postal)

This library provides [rust-lang/rust-bindgen](https://github.com/rust-lang/rust-bindgen) generated Rust <-> C bindings, and puts an ergonomic and safe Rust API on top of them.

## Installation

Follow the README instructions at [openvenues/libpostal](https://github.com/openvenues/libpostal) to install the shared library for your platform. Currently, the compiled object is dynamically linked when your project runs - static linkage could be supported in the future.

Add `postal` to your `Cargo.toml`:

Add this to your Cargo.toml:
```
[dependencies]
postal = "0.2"
```

Next, add this to your crate:
```
extern crate postal;
```

## Usage example (expand_address)

*Note*: `libpostal` is not threadsafe. As a result, do not create more than one `postal::Context` per process. `Context::expand_address` and `Context::parse_address` do internal locking, and are safe to call concurrently.

This is an example of using the `expand_address` API:

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

This is how you might use the `parse_address` API:

```rust
extern crate postal;
use postal::{Context, InitOptions, ParseAddressOptions};

// initialize a context to work with
let mut ctx = Context::new();

// enable address parsing for this context
ctx.init(InitOptions{parse_address: true}).unwrap();

// these options are safe to persist and reuse between calls to `parse_address`.
// Note: `language` and `country` are technically options that libpostal will accept
// for purposes of parsing addresses, but it ignores them at present.
let mut opts = ParseAddressOptions::new();

// parse a single address into a `postal::Components` iterator
let comps = ctx.parse_address(
	"1234 Cherry Ln, Podunk TX", &mut opts)
	.unwrap();
for c in comps {
	dbg!(c);
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
* 0.2.2
	* Resolve locking issue due to unbound Mutex guard.

* 0.2.1
	* Make Component fields public.

* 0.2.0
	* Added `parse_address` support.

* 0.1.0
	* Initial release.

## Meta

Distributed under the MIT license. See ``LICENSE`` for more information.

## Contributing

1. Fork it (<https://github.com/pnordahl/rust-postal/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
