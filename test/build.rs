use std::{env, path::PathBuf};

fn main() {
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("orig/src/MurmurHash2.cpp")
        .file("src/murmur2.cpp")
        .flag("-Wno-implicit-fallthrough")
        .debug(true);
    #[cfg(feature = "fuzz")]
    build
        .compiler("clang")
        .flag("-fno-omit-frame-pointer")
        .flag("-fsanitize=fuzzer,address");
    build.compile("murmurhash2-cpp");

    println!("cargo:rustc-link-lib=bz2");

    let bindings = bindgen::Builder::default()
        .header("src/murmur2.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
