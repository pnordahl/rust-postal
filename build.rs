// Knockway Inc. and its affiliates. All Rights Reserved

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=dylib=postal");

    let bindings = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .header("wrapper.h")
        .derive_debug(true)
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
