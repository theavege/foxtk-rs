use std::{env, path::Path};

const COMPILE: &str = "cfoxtk";
const CAPI: &str = "src/foxtk.cpp";

#[cfg(target_os = "linux")]
fn compile() -> Vec<String> {
    let library = "fox";
    println!("cargo:rust-link-lib=dylib={library}");
    let mut includes = Vec::new();
    match pkg_config::probe_library(library) {
        Ok(lib) => {
            for dir in lib.include_paths {
                includes.push(format!("-I{}", dir.display()));
            }
        }
        Err(e) => {
            eprintln!("Failed to find {library}: {e}");
            std::process::exit(1);
        }
    }

    cc::Build::new()
        .cpp(true)
        .file(CAPI)
        .flags(&includes)
        .compile(COMPILE);
    println!("cargo:rerun-if-changed={CAPI}");

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
        std::fs::write(&zip, response.bytes().expect("Failed to read response"))
            .expect("Failed to write zip");
        zip_extract::extract(
            std::fs::File::open(zip).expect("Failed to open zip"),
            &extract_dir,
            true,
        )
        .expect("Failed to extract zip");
    }

    cmake::Config::new("src")
        .env("FOX_PATH", extract_dir)
        .generator("Ninja")
        .build_target("all")
        .always_configure(true)
        .build();

    println!("cargo:rustc-link-search=native={}/build", out);
    println!("cargo:rustc-link-lib=static={COMPILE}");
    println!("cargo:rerun-if-env-changed={CAPI}");

    Vec::new()
}

fn main() {
    bindgen::Builder::default()
        .header("src/foxtk.h")
        .clang_args(compile())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
