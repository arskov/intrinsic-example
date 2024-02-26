fn main() {
    println!("cargo:rerun-if-changed=src/simd_helper.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/simd_helper.c")
        .flag("-mavx512f")
        .flag("-mavx512fp16")
        .compile("simd_helper");
    // Tell cargo to link the object file
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=static=simd_helper");
}
