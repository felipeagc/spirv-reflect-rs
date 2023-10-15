#[cfg(feature = "generate_bindings")]
extern crate bindgen;
extern crate cc;

use std::env;

fn main() {
    let mut build = cc::Build::new();

    build.include("src");

    // Add the files we build
    let source_files = ["vendor/spirv_reflect.c"];

    for source_file in &source_files {
        build.file(&source_file);
    }

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-sign-compare")
            .flag("-Wno-deprecated");
    }

    build.compile("spirv_reflect_c");

    generate_bindings("gen/bindings.rs");
}

#[cfg(feature = "generate_bindings")]
fn generate_bindings(output_file: &str) {
    let bindings = bindgen::Builder::default()
        .header("vendor/spirv_reflect.h")
        .size_t_is_usize(true)
        .formatter(bindgen::Formatter::Rustfmt)
        .blocklist_type("__darwin_.*")
        .allowlist_var("SPV.*")
        .allowlist_type("Spv.*")
        .allowlist_function("spv.*")
        .trust_clang_mangling(false)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings!");

    bindings
        .write_to_file(std::path::Path::new(output_file))
        .expect("Unable to write bindings!");
}

#[cfg(not(feature = "generate_bindings"))]
fn generate_bindings(_: &str) {}
