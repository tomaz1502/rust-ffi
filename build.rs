extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("include")
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("foo.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=foo");
    println!("cargo:rerun-if-changed={}", headers_path_str);

    let bindings = bindgen::Builder::default()
        .header("include/foo.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
