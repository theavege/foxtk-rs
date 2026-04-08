use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use glob;

#[cfg(target_os = "linux")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let zip_url = "http://fox-toolkit.org/ftp/fox-1.6.59.zip";
    let zip_path = Path::new(&out_dir).join("fox-1.6.59.zip");
    let extract_dir = Path::new(&out_dir).join("fox-1.6.59");

    if !extract_dir.exists() {
        let response = reqwest::blocking::get(zip_url).expect("Failed to download fox zip");
        let mut file = File::create(&zip_path).expect("Failed to create zip file");
        file.write_all(&response.bytes().expect("Failed to read response")).expect("Failed to write zip");
        zip_extract::extract(File::open(&zip_path).expect("Failed to open zip"), &extract_dir, true).expect("Failed to extract zip");
    }

    let mut source_paths = Vec::new();
    for entry in glob::glob(&format!("{}/src/*.cpp", extract_dir.display())).expect("Failed to read glob pattern") {
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
        .includes([extract_dir.join("include")])
        .files(source_paths)
        .compile("fox");

    let mut include_paths = Vec::from(["-Icfoxtk".to_string()]);
    include_paths.push(format!("-I{}", extract_dir.join("include").display()));
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
    let out_dir = env::var("OUT_DIR").unwrap();
    let zip_url = "http://fox-toolkit.org/ftp/fox-1.6.59.zip";
    let zip_path = Path::new(&out_dir).join("fox-1.6.59.zip");
    let extract_dir = Path::new(&out_dir).join("fox-1.6.59");

    if !extract_dir.exists() {
        let response = reqwest::blocking::get(zip_url).expect("Failed to download fox zip");
        let mut file = File::create(&zip_path).expect("Failed to create zip file");
        file.write_all(&response.bytes().expect("Failed to read response")).expect("Failed to write zip");
        zip_extract::extract(File::open(&zip_path).expect("Failed to open zip"), &extract_dir, true).expect("Failed to extract zip");
    }

    let mut source_paths = Vec::new();
    for entry in glob::glob(&format!("{}/src/*.cpp", extract_dir.display())).expect("Failed to read glob pattern") {
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
        .includes([extract_dir.join("include")])
        .files(source_paths)
        .define("WIN32", None)
        .define("_WINDOWS", None)
        .define("UNICODE", None)
        .compile("fox");
    const CAPI: &str = "cfoxtk/foxtk.cpp";
    cc::Build::new().cpp(true).file(CAPI).compile("cfoxtk");
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
