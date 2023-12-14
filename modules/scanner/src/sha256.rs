use std::io::Read;

use sha2::{
    Digest,
    Sha256,
};

use crate::ScanError;

pub fn calc_sha256(file: &mut std::fs::File) -> Result<String, ScanError> {
    // Create a SHA-256 "hasher"
    let mut hasher = Sha256::new();

    // Read the file in 4KB chunks and feed them to the hasher
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    //Ok(format!("{:x}", hasher.finalize()))
    Ok(String::from("elo"))
}
