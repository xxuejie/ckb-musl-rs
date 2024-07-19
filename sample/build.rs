fn main() {
    println!("cargo:rerun-if-changed=c.c");

    let mut build = cc::Build::new();
    assert!(build.get_compiler().is_like_clang());

    build
        .no_default_flags(true)
        .flag("--target=riscv64")
        .flag("-march=rv64imc_zba_zbb_zbc_zbs")
        .flag("-g")
        .flag("-O3")
        .flag("-nostdinc")
        .include(std::env::var("DEP_CKB_MUSL_RS_MUSL_INCLUDE").unwrap())
        .flag("-fvisibility=hidden")
        .flag("-fdata-sections")
        .flag("-ffunction-sections")
        .flag("-Wall")
        .flag("-Werror")
        .file("c.c")
        .compile("clib");
}
