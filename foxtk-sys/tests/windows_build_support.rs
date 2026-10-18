#[path = "../build_support.rs"]
mod build_support;

use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use flate2::{Compression, write::GzEncoder};
use sha2::{Digest, Sha256};
use tar::{Builder, Header};

const DIST: &str = "fox-1.6.59";

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "foxtk-build-support-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn archive_with_path(path: &str) -> Vec<u8> {
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut archive = Builder::new(encoder);
    let source = b"int fox_archive_probe;\n";
    let mut header = Header::new_gnu();
    header.set_size(source.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive.append_data(&mut header, path, &source[..]).unwrap();
    archive.into_inner().unwrap().finish().unwrap()
}

#[test]
fn extracts_and_locates_fox_sources() {
    let temp = TempDir::new();
    let archive = archive_with_path(&format!("{DIST}/src/probe.cpp"));
    let digest = format!("{:x}", Sha256::digest(&archive));

    let source_dir = build_support::extract_fox_archive(&archive, &temp.0, DIST, &digest).unwrap();

    assert_eq!(source_dir, temp.0.join(DIST));
    assert!(source_dir.join("src/probe.cpp").is_file());
}

#[test]
fn rejects_an_archive_with_the_wrong_checksum() {
    let temp = TempDir::new();
    let archive = archive_with_path(&format!("{DIST}/src/probe.cpp"));

    let error =
        build_support::extract_fox_archive(&archive, &temp.0, DIST, "wrong-checksum").unwrap_err();

    assert!(error.to_string().contains("SHA-256 mismatch"));
    assert!(!temp.0.join(DIST).exists());
}

#[test]
fn locates_nested_fox_source_layouts() {
    let temp = TempDir::new();
    let nested_source_dir = temp.0.join(DIST).join(DIST).join("src");
    fs::create_dir_all(&nested_source_dir).unwrap();

    assert_eq!(
        build_support::fox_source_dir(&temp.0.join(DIST), DIST).unwrap(),
        temp.0.join(DIST).join(DIST)
    );
}
