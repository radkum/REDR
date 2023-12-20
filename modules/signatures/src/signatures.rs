use std::{
    collections::BTreeSet,
    io::Write,
};

pub use crate::error::SignatureError;
use crate::sha256::{
    sha256_from_path,
    Sha256,
};

pub struct Signatures {
    signatures: BTreeSet<Sha256>,
}

impl Signatures {
    pub(crate) fn read_sig_file(path: &str) -> Result<Self, SignatureError> {
        let data_vec = std::fs::read(path)?;
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
        out.write_all(&bytes)?;
        Ok(())
    }

    // pub(crate) fn create_rsig_file_from_msig(rsig_file_path: &str) -> Result<Self,
    // SignaturesError> {     Ok(Self {
    //         signatures: BTreeMap::from([(
    //             "elo".to_string(),
    //             FileInfo::Malware(MalwareInfo { desc: "test malware".to_string(), action:
    // Action::Delete }),         )]),
    //     })
    // }

    pub fn match_(&self, sha: Sha256) -> Result<bool, SignatureError> {
        Ok(self.signatures.contains(&sha))
    }
}
