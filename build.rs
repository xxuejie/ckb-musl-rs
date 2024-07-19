use std::env;
use std::path::PathBuf;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch != "riscv64" {
        return;
    }

    let base_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let musl_include_dir = base_dir
        .join("deps")
        .join("musl")
        .join("release")
        .join("include");
    println!("cargo:musl_include={}", musl_include_dir.to_str().unwrap());
    let musl_lib_dir = base_dir
        .join("deps")
        .join("musl")
        .join("release")
        .join("lib");
    println!(
        "cargo::rustc-link-search={}",
        musl_lib_dir.to_str().unwrap()
    );
    println!("cargo::rustc-link-lib=c");
    println!("cargo::rustc-link-lib=gcc");

    if cfg!(feature = "link-against-builtins") {
        let builtin_lib_dir = base_dir.join("deps").join("builtins").join("build");
        println!(
            "cargo::rustc-link-search={}",
            builtin_lib_dir.to_str().unwrap()
        );
        println!("cargo::rustc-link-lib=static=compiler-rt");
    }
}
