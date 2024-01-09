use std::io;

use sha3::Digest;

const SHA256_LEN: usize = 32;
pub type Sha256 = [u8; SHA256_LEN];

//pub type Sha256_String = Sha256;

pub fn sha256_from_file_pointer(file: &mut impl io::Read) -> Result<Sha256, io::Error> {
    // Create a SHA-256 "hasher"
    let mut hasher = sha3::Sha3_256::new();

    // Read the file in 4KB chunks and feed them to the hasher
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let mut checksum_buf = Sha256::default();
    checksum_buf.copy_from_slice(&hasher.finalize()[..]);
    Ok(checksum_buf)
}

pub fn sha256_from_path(file_path: &str) -> Result<Sha256, io::Error> {
    let mut file = std::fs::File::open(file_path)?;
    sha256_from_file_pointer(&mut file)
}
