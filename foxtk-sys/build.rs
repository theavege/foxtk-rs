use std::{env, path::Path};

const CAPI: &str = "cfoxtk/foxtk.cpp";

#[cfg(target_os = "linux")]
fn compile() -> Vec<String> {
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

    cc::Build::new()
        .cpp(true)
        .flags(&include_paths)
        .file(CAPI)
        .compile("cfoxtk");

    println!("cargo:rerun-if-changed={CAPI}");
    include_paths
}

#[cfg(target_os = "windows")]
fn compile() -> Vec<String> {
    use std::{fs::File, io::Write};
    const DIST: &str = "fox-snapshot";
    let zip_url = format!("http://fox-toolkit.org/ftp/{DIST}.zip");
    let out_dir = env::var("OUT_DIR").unwrap();
    let zip_path = Path::new(&out_dir).join(format!("{DIST}.zip"));
    let extract_dir = Path::new(&out_dir).join(DIST);
    if !extract_dir.exists() {
        let mut file = File::create(&zip_path).expect("Failed to create zip file");
        let response = reqwest::blocking::get(zip_url).expect("Failed to download fox zip");
        file.write_all(&response.bytes().expect("Failed to read response"))
            .expect("Failed to write zip");
        zip_extract::extract(
            File::open(&zip_path).expect("Failed to open zip"),
            &extract_dir,
            true,
        )
        .expect("Failed to extract zip");
    }

    let mut source_paths = Vec::new();
    for entry in glob::glob(&format!("{}/lib/*.cpp", extract_dir.display()))
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => source_paths.push(path),
            Err(e) => eprintln!("cargo:warning=Glob error: {:?}", e),
        }
    }
    let include_paths = Vec::from([
        format!("-I{}", extract_dir.join("include").display()),
    ]);
    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .file(CAPI)
        .files(&source_paths)
        .flags(&include_paths)
        .compile("cfoxtk");
    Vec::from([
        "-Icfoxtk".to_string(),
    ])
}

fn main() {
    bindgen::Builder::default()
        .header("cfoxtk/foxtk.h")
        .clang_args(compile())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}
