use std::io;

use sha2::Digest;

const SHA256_LEN: usize = 32;
pub type Sha256Buff = [u8; SHA256_LEN];

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaError {
    #[error("Sha string must have 32 chars not {string_len}")]
    IncorrectStringLen { string_len: usize },
    #[error("ToHex error: {0}")]
    ToHexError(#[from] hex::FromHexError),
}

pub fn sha256_from_file_pointer(file: &mut impl io::Read) -> Result<Sha256Buff, io::Error> {
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

    let mut checksum_buf = Sha256Buff::default();
    checksum_buf.copy_from_slice(&hasher.finalize()[..]);
    Ok(checksum_buf)
}

pub fn sha256_from_vec(v: Vec<u8>) -> Sha256Buff {
    let mut hasher = sha2::Sha256::new();
    hasher.update(v);

    let mut checksum_buf = Sha256Buff::default();
    checksum_buf.copy_from_slice(&hasher.finalize()[..]);
    checksum_buf
}

pub fn sha256_from_vec_of_vec(vec: Vec<Vec<u8>>) -> Result<Sha256Buff, io::Error> {
    let mut hasher = sha2::Sha256::new();

    for v in vec {
        hasher.update(v);
    }

    let mut checksum_buf = Sha256Buff::default();
    checksum_buf.copy_from_slice(&hasher.finalize()[..]);
    Ok(checksum_buf)
}

pub fn sha256_from_path(file_path: &str) -> Result<Sha256Buff, io::Error> {
    let mut file = std::fs::File::open(file_path)?;
    sha256_from_file_pointer(&mut file)
}

pub fn convert_string_to_sha256(s: &str) -> Result<Sha256Buff, ShaError> {
    let mut sha = Sha256Buff::default();
    let v = hex::decode(s)?;

    if v.len() != SHA256_LEN {
        return Err(ShaError::IncorrectStringLen { string_len: v.len() });
    }
    sha.copy_from_slice(&v);
    Ok(sha)
}

pub fn convert_sha256_to_string(sha: &Sha256Buff) -> String {
    hex::encode_upper(sha)
}
