use std::collections::BTreeMap;

use crate::{
    error::SignaturesError,
    file_info::{
        FileInfo,
        FileInfo::Malware,
        MalwareInfo,
    },
};

pub type sha256 = String;

pub(crate) struct Signatures {
    signatures: BTreeMap<sha256, FileInfo>,
}

impl Signatures {
    pub(crate) fn read_sig_file(path: &str) -> Result<Self, SignaturesError> {
        Ok(Self {
            signatures: BTreeMap::from([(
                "elo".to_string(),
                FileInfo::Malware(MalwareInfo { desc: "test malware".to_string() }),
            )]),
        })
    }

    pub(crate) fn match_(&self, sha: sha256) -> Result<&FileInfo, SignaturesError> {
        Ok(self.signatures.get(&sha).unwrap_or(&FileInfo::Unknown))
    }
}
