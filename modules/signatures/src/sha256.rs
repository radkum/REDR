use std::io::Read;

use sha2::Digest;

const SHA256_LEN: usize = 32;
pub type Sha256 = [u8; SHA256_LEN];

pub type Sha256_String = String;

pub fn sha256_from_file_pointer(file: &mut std::fs::File) -> Result<Sha256_String, std::io::Error> {
    // Create a SHA-256 "hasher"
    let mut hasher = sha2::Sha256::new();

    // Read the file in 4KB chunks and feed them to the hasher
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

pub fn sha256_from_path(file_path: &str) -> Result<Sha256_String, std::io::Error> {
    let mut file = std::fs::File::open(file_path)?;
    sha256_from_file_pointer(&mut file)
}
