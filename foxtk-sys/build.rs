use std::{env, path::Path};

const COMPILE: &str = "cfoxtk";
const LIBRARY: &str = "fox";
const CAPI: &str = "cfoxtk/foxtk.cpp";

#[cfg(target_os = "linux")]
fn compile() -> Vec<String> {
    println!("cargo:rust-link-lib=dylib={LIBRARY}");
    let mut includes = Vec::new();
    match pkg_config::probe_library(LIBRARY) {
        Ok(lib) => {
            for dir in lib.include_paths {
                includes.push(format!("-I{}", dir.display()));
            }
        }
        Err(e) => {
            eprintln!("Failed to find {LIBRARY}: {e}");
            std::process::exit(1);
        }
    }

    cc::Build::new()
        .cpp(true)
        .file(CAPI)
        .flags(&includes)
        .compile(COMPILE);

    includes
}

#[cfg(target_os = "windows")]
fn compile() -> Vec<String> {
    const DIST: &str = "fox-snapshot";
    let url = format!("http://fox-toolkit.org/ftp/{DIST}.zip");
    let out = env::var("OUT_DIR").unwrap();
    let zip = Path::new(&out).join(format!("{DIST}.zip"));
    let extract_dir = Path::new(&out).join(DIST);

    if !extract_dir.exists() {
        let response = reqwest::blocking::get(url).expect("Failed to download fox zip");
        std::fs::write(&zip, &response.bytes().expect("Failed to read response"))
            .expect("Failed to write zip");
        zip_extract::extract(
            std::fs::File::open(zip).expect("Failed to open zip"),
            &extract_dir,
            true,
        )
        .expect("Failed to extract zip");
    }

    let mut sources = Vec::new();
    for entry in glob::glob(&format!("{}/lib/*.cpp", extract_dir.display()))
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => sources.push(path),
            Err(e) => eprintln!("cargo:warning=Glob error: {:?}", e),
        }
    }

    let include = extract_dir.join("include").display().to_string();

    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .files(&sources)
        .include(&include)
        .compile(LIBRARY);

    cc::Build::new()
        .cpp(true)
        .define("WIN32", None)
        .file(CAPI)
        .include(&include)
        .compile(COMPILE);

    Vec::new()
}

fn main() {
    bindgen::Builder::default()
        .header("cfoxtk/foxtk.h")
        .clang_args(compile())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
