use std::env;
use std::path::PathBuf;

fn main() {
    // Build the project in the path `cudd` and installs it in `$OUT_DIR`
    let dst = autotools::build("cudd");

    // Simply link the library without using pkg-config
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=cudd");

    // Tell cargo to tell rustc to link the system cudd
    // shared library.
    //println!("cargo:rustc-link-lib=cudd");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
