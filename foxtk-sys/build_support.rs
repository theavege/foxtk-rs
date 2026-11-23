use std::{
    error::Error,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use tar::Archive;

pub fn fox_source_dir(extract_dir: &Path, distribution: &str) -> Result<PathBuf, Box<dyn Error>> {
    let source_dir = extract_dir.join("src");
    if source_dir.is_dir() {
        return Ok(extract_dir.to_path_buf());
    }

    let nested_dir = extract_dir.join(distribution);
    if nested_dir.join("src").is_dir() {
        return Ok(nested_dir);
    }

    Err(format!(
        "FOX source tree was not found under {}",
        extract_dir.display()
    )
    .into())
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in Sha256::digest(bytes) {
        hex.push(HEX[(byte >> 4) as usize] as char);
        hex.push(HEX[(byte & 0x0f) as usize] as char);
    }
    hex
}

pub fn extract_fox_archive(
    archive: &[u8],
    destination: &Path,
    distribution: &str,
    expected_sha256: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let actual_sha256 = sha256_hex(archive);
    if actual_sha256 != expected_sha256 {
        return Err(format!(
            "SHA-256 mismatch for {distribution}: expected {expected_sha256}, got {actual_sha256}"
        )
        .into());
    }

    Archive::new(GzDecoder::new(archive)).unpack(destination)?;
    fox_source_dir(&destination.join(distribution), distribution)
}
