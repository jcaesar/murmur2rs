use std::{env, path::PathBuf};

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("c/src/MurmurHash2.cpp")
        .file("murmur2.cpp")
        .flag("-Wno-implicit-fallthrough")
        .compile("murmurhash2-cpp");

    println!("cargo:rustc-link-lib=bz2");

    let bindings = bindgen::Builder::default()
        .header("murmur2.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
