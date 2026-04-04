fn main() {
    let mut include_paths = Vec::from(["-Icfoxtk".to_string()]);
    for library in ["fox"] {
        match pkg_config::probe_library(library) {
            Ok(lib) => {
                for dir in lib.include_paths {
                    include_paths.push(format!("-I{}", dir.display()));
                }
            }
            Err(e) => {
                eprintln!("Failed to find {library}: {e}");
                std::process::exit(1);
            }
        }
    }
    const CAPI: &str = "cfoxtk/foxtk.cpp";
    cc::Build::new()
        .cpp(true)
        .flags(&include_paths)
        .file(CAPI)
        .compile("cfoxtk");
    println!("cargo:rerun-if-changed={CAPI}");
    bindgen::Builder::default()
        .header("cfoxtk/foxtk.h")
        .clang_args(&include_paths)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
        )
        .unwrap();
}
