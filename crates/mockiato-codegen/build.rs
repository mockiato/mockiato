use rustc_version::{version_meta, Channel};

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rustc_is_nightly)");
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=rustc_is_nightly");
    }
}
