// build.rs

use std::env;

fn main() {

    println!("cargo:rustc-link-lib=raylib"); // the "-l" flag
}