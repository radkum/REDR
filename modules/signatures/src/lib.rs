use crate::error::SignatureError;
pub use crate::signatures::Signatures;

mod error;
mod file_info;
pub mod sha256;
pub mod signatures;

pub use sha256::sha256_from_file_pointer;

pub fn get_signatures(sig_path: &str) -> Result<Signatures, SignatureError> {
    Signatures::read_sig_file(sig_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
