#[cfg(target_os = "linux")]
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
        .flag_if_supported("-std=c++14")
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

#[cfg(target_os = "windows")]
fn main() {
    let mut source_paths = Vec::new();
    for entry in glob::glob("fox-1.6.59/src/*.cpp").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let filename = path.file_name().unwrap().to_string_lossy();
                if filename != "vsscanf.cpp" {
                    source_paths.push(path);
                };
            }
            Err(e) => println!("cargo:warning=Glob error: {:?}", e),
        }
    }
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .flag_if_supported("/EHsc")
        .includes(["fox-1.6.59/include"])
        .files(source_paths)
        .define("WIN32", None)
        .define("_WINDOWS", None)
        .define("UNICODE", None)
        .compile("fox");
    const CAPI: &str = "cfoxtk/foxtk.cpp";
    cc::Build::new()
        .cpp(true)
        .file(CAPI)
        .compile("cfoxtk");
    println!("cargo:rerun-if-changed={CAPI}");
    bindgen::Builder::default()
        .header("cfoxtk/foxtk.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
        )
        .unwrap();
}
