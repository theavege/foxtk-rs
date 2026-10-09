use std::{env, path::Path, error::Error};

const COMPILE: &str = "cfoxtk";
const CAPI: &str = "src/foxtk.cpp";

#[cfg(target_os = "linux")]
fn compile() -> Result<Vec<String>, Box<dyn Error>> {
    let library = "fox";
    println!("cargo:rust-link-lib=dylib={library}");
    let mut includes = Vec::new();
    for dir in pkg_config::probe_library(library)?.include_paths {
        includes.push(format!("-I{}", dir.display()));
    }
    cc::Build::new()
        .cpp(true)
        .file(CAPI)
        .flags(&includes)
        .compile(COMPILE);
    println!("cargo:rerun-if-changed={CAPI}");
    Ok(includes)
}

#[cfg(not(target_os = "linux"))]
fn compile() -> Result<Vec<String>, Box<dyn Error>> {
    const DIST: &str = "fox-1.6.59";
    let url = format!("http://fox-toolkit.org/ftp/{DIST}.zip");
    let out = env::var("OUT_DIR")?;
    let zip = Path::new(&out).join(format!("{DIST}.zip"));
    let extract_dir = Path::new(&out).join(DIST);

    if !extract_dir.exists() {
        let response = reqwest::blocking::get(url)?;
        std::fs::write(&zip, response.bytes()?)?;
        zip_extract::extract(
            std::fs::File::open(zip)?,
            &extract_dir,
            true,
        )?;
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

    Ok(Vec::new())
}

fn main() -> Result<(), Box<dyn Error>> {
    bindgen::Builder::default()
        .header("src/foxtk.h")
        .clang_args(compile()?)
        .generate()?
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))?;
    Ok(())
}
