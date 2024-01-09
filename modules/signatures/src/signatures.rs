use std::{
    collections::BTreeSet,
    io::Write,
};

use common::redr;

pub use crate::error::SignatureError;
use crate::{
    file_info::FileInfo,
    sha256::{
        sha256_from_path,
        Sha256,
    },
};

//const MSET_MAGIC_U32: u32 = 0x5445354D; //M5ET
const MSET_MAGIC: [u8; 4] = [0x4D, 0x35, 0x45, 0x54]; //M5ET
pub struct Signatures {
    signatures: BTreeSet<Sha256>,
}

impl Signatures {
    pub(crate) fn read_sig_file(path: &str) -> Result<Self, SignatureError> {
        //read magic
        let data_vec = std::fs::read(path)?;
        if data_vec.len() <= std::mem::size_of_val(&MSET_MAGIC) {
            return Err(SignatureError::InvalidMalsetSizeError {});
        }

        let data_vec = if let Some(data_vec) = data_vec.strip_prefix(&MSET_MAGIC) {
            data_vec
        } else {
            return Err(SignatureError::InvalidMalsetSizeError {});
        };

        if data_vec.len() % std::mem::size_of::<Sha256>() != 0 {
            return Err(SignatureError::InvalidMalsetSizeError {});
        }
        let sha_vec: &[Sha256] = unsafe {
            let bytes_len = data_vec.len() / std::mem::size_of::<Sha256>();
            let data_ptr = data_vec.as_ptr() as *const Sha256;
            &*std::ptr::slice_from_raw_parts(data_ptr, bytes_len)
        };
        let malset = BTreeSet::<Sha256>::from_iter(sha_vec.to_vec());
        Ok(Self { signatures: malset })
    }

    pub fn create_msig_file(path_to_dir: &str, out_file: &str) -> Result<(), SignatureError> {
        let paths = std::fs::read_dir(path_to_dir).unwrap();

        let mut mset = BTreeSet::new();
        for path in paths {
            let path = path?;
            //log::trace!("path: {:?}", &path);
            if path.file_type()?.is_file() {
                let sha =
                    sha256_from_path(path.path().into_os_string().into_string().unwrap().as_str())?;
                mset.insert(sha);
                log::trace!("path: {:?}", &path);
            }
        }

        log::info!("mset size: {}", mset.len());
        log::trace!("mset: {:?}", mset);

        let sha256_vec = Vec::<Sha256>::from_iter(mset);
        let bytes = unsafe {
            let bytes_len = sha256_vec.len() * std::mem::size_of::<Sha256>();
            let data_ptr = sha256_vec.as_ptr() as *const u8;
            &*std::ptr::slice_from_raw_parts(data_ptr, bytes_len)
        };

        let mut out = std::fs::File::create(out_file)?;
        out.write_all(&MSET_MAGIC)?;
        out.write_all(&bytes)?;
        Ok(())
    }

    pub fn eval_file(&self, file: &mut redr::FileReader) -> Result<FileInfo, SignatureError> {
        let sha256 = crate::sha256::sha256_from_file_pointer(file)?;
        let sha_str = hex::encode_upper(&sha256);

        let file_info = self.match_(sha256)?;
        println!("\"{sha_str}\" -> {:?}", &file_info);

        Ok(file_info)
    }

    pub fn match_(&self, sha: Sha256) -> Result<FileInfo, SignatureError> {
        let info = match self.signatures.contains(&sha) {
            true => FileInfo::Malicious,
            false => FileInfo::Clean,
        };

        Ok(info)
    }
}
