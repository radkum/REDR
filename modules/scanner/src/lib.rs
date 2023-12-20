pub mod error;

use signatures::{
    sha256_from_file_pointer,
    signatures::Signatures,
};

use crate::error::ScanError;

pub fn run_scanner(file_path: &str, sigs: Signatures) -> Result<(), ScanError> {
    log::debug!("SCANNER begin");
    let mut file = std::fs::File::open(file_path)?;

    let sha256 = sha256_from_file_pointer(&mut file)?;
    let sha_str = hex::encode_upper(&sha256);

    let res = if sigs.match_(sha256)? { "Matched" } else { "Not matched" };

    println!("{res}, sha256: \"{sha_str}\"");

    log::debug!("SCANNER end");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
