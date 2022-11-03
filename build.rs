extern crate bindgen;
extern crate pkg_config;

use std::path::PathBuf;

fn main() {
    let config = pkg_config::Config::new()
        .probe("libdrm")
        .unwrap()
        .include_paths;

    let config: Vec<String> = config
        .iter()
        .map(|path| format!("-I{}", path.to_str().unwrap()))
        .collect();

    println!("cargo:rustc-link-lib=drm");
    println!("cargo:rustc-link-lib=drm_amdgpu");

    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        // .no_unstable_rust()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper/wrapper_drm.h")
        .header("wrapper/wrapper_amdgpu.h")
        .clang_args(config.iter())
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings: {wrapper_name}");

    //let out_path = PathBuf::from(env!("OUT_DIR"));
    let out_path = PathBuf::from("./bindings/");

    bindings
        .write_to_file(out_path.join(&format!("drm.rs")))
        .expect("Couldn't write bindings!");
}
