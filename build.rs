use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("msp430fr4133.ld"))
        .unwrap()
        .write_all(include_bytes!("ldscripts/msp430fr4133.ld"))
        .unwrap();
    File::create(out.join("msp430fr4133_symbols.ld"))
        .unwrap()
        .write_all(include_bytes!("ldscripts/msp430fr4133_symbols.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ldscripts/msp430fr4133.ld");
    println!("cargo:rerun-if-changed=ldscripts/msp430fr4133_symbols.ld");
}
