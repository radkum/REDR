pub mod error;
mod file_info;
mod sha256;
mod signatures;

use std::io::Read;

pub use crate::error::ScanError;
use crate::{
    sha256::calc_sha256,
    signatures::Signatures,
};

pub fn run_scanner(file_path: &str) -> Result<(), ScanError> {
    log::info!("SCANNER begin");
    let mut file = std::fs::File::open(file_path)?;

    let sha256 = calc_sha256(&mut file)?;
    let sigs = Signatures::read_sig_file("asdfasd")?;

    println!("{:?}", sigs.match_(sha256)?);

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
