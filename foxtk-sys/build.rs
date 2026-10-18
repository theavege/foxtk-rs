use std::{env, error::Error, path::Path};

const COMPILE: &str = "cfoxtk";
const CAPI: &str = "src/foxtk.cpp";

#[cfg(target_os = "linux")]
fn compile() -> Result<Vec<String>, Box<dyn Error>> {
    let library = "fox";
    println!("cargo:rustc-link-lib=dylib={library}");
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
    println!("cargo:rerun-if-changed=src/foxtk.h");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(includes)
}

#[cfg(target_os = "windows")]
fn compile() -> Result<Vec<String>, Box<dyn Error>> {
    use flate2::read::GzDecoder;
    use sha2::{Digest, Sha256};
    use tar::Archive;

    const DIST: &str = "fox-1.6.59";
    const SHA256: &str = "48f33d2dd5371c2d48f6518297f0ef5bbf3fcd37719e99f815dc6fc6e0f928ae";
    let url = format!("https://distfiles.macports.org/fox/{DIST}.tar.gz");
    let out = env::var("OUT_DIR")?;
    let archive_path = Path::new(&out).join(format!("{DIST}.tar.gz"));
    let extract_dir = Path::new(&out).join(DIST);

    if !extract_dir.exists() {
        let response = reqwest::blocking::get(url)?.error_for_status()?;
        let archive = response.bytes()?;
        let digest = format!("{:x}", Sha256::digest(&archive));
        if digest != SHA256 {
            return Err(
                format!("SHA-256 mismatch for {DIST}: expected {SHA256}, got {digest}").into(),
            );
        }
        std::fs::write(&archive_path, &archive)?;
        let decoder = GzDecoder::new(std::fs::File::open(archive_path)?);
        Archive::new(decoder).unpack(&out)?;
    }
    let fox_path = if extract_dir.join("src").is_dir() {
        extract_dir.clone()
    } else {
        extract_dir.join(DIST)
    };
    let cmake_cache = Path::new(&out).join("build/CMakeCache.txt");
    if cmake_cache.exists() && !std::fs::read_to_string(&cmake_cache)?.contains("FOX_PATH:") {
        std::fs::remove_file(&cmake_cache)?;
    }

    cmake::Config::new("src")
        .define("FOX_PATH", fox_path)
        .generator("Ninja")
        .build_target("all")
        .always_configure(false)
        .build();
    println!(
        "cargo:warning=foxtk-sys-cmake-build-dir={}",
        Path::new(&out).join("build").display()
    );
    println!("cargo:rerun-if-env-changed=FOXTK_CMAKE_CACHE_CHECK");

    println!("cargo:rustc-link-search=native={}/build", out);
    println!("cargo:rustc-link-lib=static={COMPILE}");
    for library in [
        "AdvAPI32", "ComCtl32", "Gdi32", "Imm32", "Mpr", "MSImg32", "User32", "WinSpool", "WSock32",
    ] {
        println!("cargo:rustc-link-lib=dylib={library}");
    }
    println!("cargo:rerun-if-changed={CAPI}");
    println!("cargo:rerun-if-changed=src/foxtk.h");
    println!("cargo:rerun-if-changed=src/CMakeLists.txt");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(Vec::new())
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn compile() -> Result<Vec<String>, Box<dyn Error>> {
    Err(format!(
        "foxtk-sys currently supports Linux and Windows; target OS is {}",
        env::var("CARGO_CFG_TARGET_OS")?
    )
    .into())
}

fn main() -> Result<(), Box<dyn Error>> {
    bindgen::Builder::default()
        .header("src/foxtk.h")
        .clang_args(compile()?)
        .generate()?
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs"))?;
    Ok(())
}
