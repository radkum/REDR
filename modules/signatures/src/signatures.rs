use std::collections::{
    BTreeMap,
    BTreeSet,
};

pub use crate::error::SignatureError;
use crate::{
    file_info::{
        Action,
        FileInfo,
        FileInfo::Malware,
        MalwareInfo,
    },
    sha256::{
        sha256_from_path,
        Sha256_String,
    },
};

pub struct Signatures {
    signatures: BTreeMap<Sha256_String, FileInfo>,
}

impl Signatures {
    pub(crate) fn read_sig_file(path: &str) -> Result<Self, SignatureError> {
        Ok(Self {
            signatures: BTreeMap::from([(
                "elo".to_string(),
                FileInfo::Malware(MalwareInfo {
                    desc: "test malware".to_string(),
                    action: Action::Delete,
                }),
            )]),
        })
    }

    pub fn create_msig_file(path_to_dir: &str, _out_dir: &str) -> Result<(), SignatureError> {
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

    pub fn match_(&self, sha: Sha256_String) -> Result<&FileInfo, SignatureError> {
        Ok(self.signatures.get(&sha).unwrap_or(&FileInfo::Unknown))
    }
}
