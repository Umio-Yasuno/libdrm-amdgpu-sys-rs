#[cfg(feature = "buildtime_bindgen")]
fn build() {
    use std::path::PathBuf;

    let config = pkg_config::Config::new()
        .probe("libdrm")
        .unwrap()
        .include_paths;

    let config: Vec<String> = config
        .iter()
        .map(|path| format!("-I{}", path.to_str().unwrap()))
        .collect();

    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        // .no_unstable_rust()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper/wrapper_drm.h")
        .header("wrapper/wrapper_amdgpu.h")
        .header("wrapper/wrapper_gpu_metrics.h")
        .clang_args(config.iter())
        .use_core()
        .ctypes_prefix("::core::ffi")
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

    convert_amdgpu_ids();
}

fn convert_amdgpu_ids() {
    use std::fmt::Write;
    const AMDGPU_IDS: &str = include_str!("bindings/amdgpu.ids");

    let mut s = String::from("pub const AMDGPU_IDS: &[(u32, u32, &str)] = &[\n");

    for line in AMDGPU_IDS.lines() {
        if line.starts_with('#') { continue }

        let mut split = line.split(",\t");

        if let [Some(did), Some(rid), Some(name)] = [split.next(), split.next(), split.next()] {
            writeln!(s, "    (0x{did}, 0x{rid}, {name:?}),").unwrap();
        }
    }

    writeln!(s, "];").unwrap();

    std::fs::write("bindings/amdgpu_ids.rs", s).unwrap();
}

fn main() {
    #[cfg(feature = "link-drm")]
    println!("cargo:rustc-link-lib=drm");
    #[cfg(feature = "link-drm")]
    println!("cargo:rustc-link-lib=drm_amdgpu");

    #[cfg(feature = "buildtime_bindgen")]
    build();
}
