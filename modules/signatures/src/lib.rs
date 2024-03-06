pub mod error;
mod file_info;
pub mod sha256;
mod malware_set;

pub use sha256::sha256_from_file_pointer;
use crate::error::MsetError;
pub use crate::malware_set::MalwareSet;
use crate::malware_set::MsetDeserializer;
pub use crate::malware_set::MsetSerializer;

pub fn get_malware_set(mset_path: &str) -> Result<MalwareSet, MsetError> {
    let des = MsetDeserializer::new(mset_path)?;
    des.get_malset()
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
