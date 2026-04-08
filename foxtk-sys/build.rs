use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const DIST: &str = "fox-1.6.59";

#[cfg(target_os = "linux")]
fn main() {
    let zip_url = format!("http://fox-toolkit.org/ftp/{DIST}.zip");
    let out_dir = env::var("OUT_DIR").unwrap();
    let zip_path = Path::new(&out_dir).join(format!("{DIST}.zip"));
    let extract_dir = Path::new(&out_dir).join(DIST);

    if !extract_dir.exists() {
        let response = reqwest::blocking::get(zip_url).expect("Failed to download fox zip");
        let mut file = File::create(&zip_path).expect("Failed to create zip file");
        file.write_all(&response.bytes().expect("Failed to read response")).expect("Failed to write zip");
        zip_extract::extract(File::open(&zip_path).expect("Failed to open zip"), &extract_dir, true).expect("Failed to extract zip");
    }

    let mut source_paths = Vec::new();
    for entry in glob::glob(&format!("{}/src/*.cpp", extract_dir.display())).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => source_paths.push(path),
            Err(e) => eprintln!("cargo:warning=Glob error: {:?}", e),
        }
    }
    const CAPI: &str = "cfoxtk/foxtk.cpp";
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .includes([extract_dir.join("include")])
        .files(source_paths)
        .flags(&include_paths)
        .file(CAPI)
        .compile("cfoxtk");
    println!("cargo:rerun-if-changed={CAPI}");
    bindgen::Builder::default()
        .header("cfoxtk/foxtk.h")
        .clang_args(&["-Icfoxtk".to_string(), format!("-I{}", extract_dir.join("include").display())])
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
        )
        .unwrap();
}

#[cfg(target_os = "windows")]
fn main() {
    let zip_url = format!("http://fox-toolkit.org/ftp/{DIST}.zip");
    let out_dir = env::var("OUT_DIR").unwrap();
    let zip_path = Path::new(&out_dir).join(format!("{DIST}.zip"));
    let extract_dir = Path::new(&out_dir).join(DIST);

    if !extract_dir.exists() {
        let mut file = File::create(&zip_path).expect("Failed to create zip file");
        let response = reqwest::blocking::get(zip_url).expect("Failed to download fox zip");
        file.write_all(&response.bytes().expect("Failed to read response")).expect("Failed to write zip");
        zip_extract::extract(File::open(&zip_path).expect("Failed to open zip"), &extract_dir, true).expect("Failed to extract zip");
    }

    let mut source_paths = Vec::new();
    for entry in glob::glob(&format!("{}/src/*.cpp", extract_dir.display())).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => source_paths.push(path),
            Err(e) => eprintln!("cargo:warning=Glob error: {:?}", e),
        }
    }
    const CAPI: &str = "cfoxtk/foxtk.cpp";
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .flag_if_supported("/EHsc")
        .define("WIN32", None)
        .define("HAVE_VSSCANF", None)
        .includes([extract_dir.join("include")])
        .include("cfoxtk")
        .files(source_paths)
        .file(CAPI)
        .compile("cfoxtk");
    println!("cargo:rerun-if-changed={CAPI}");
    bindgen::Builder::default()
        .header("cfoxtk/foxtk.h")
        .clang_args(&["-Icfoxtk".to_string(), format!("-I{}", extract_dir.join("include").display())])
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
        )
        .unwrap();
}
