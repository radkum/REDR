use common::{
    detection::DetectionReport,
    sha256_utils::{
        convert_sha256_to_string, convert_string_to_sha256, sha256_from_vec, Sha256Buff,
    },
};
use serde::{Deserialize, Serialize};
pub(super) use yaml_signature::{YamlSigData, YamlSignature};

use super::{Description, SigName};
use crate::SigSetError;

pub(crate) mod yaml_signature;
pub(crate) type SigId = u32;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerSigHeader {
    pub(crate) id: SigId,
    pub(crate) size: u32,
    pub(crate) offset: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SerSignature {
    header: SerSigHeader,
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SigBase {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Signature {
    pub(crate) base: SigBase,
    pub(crate) data: SigData,
}

impl Signature {
    pub(crate) fn from_yaml(yaml_sig: YamlSignature) -> Result<Self, SigSetError> {
        Ok(Self { base: yaml_sig.base, data: SigData::from_yaml(yaml_sig.data)? })
    }

    pub(crate) fn sig_data(&self) -> &SigData {
        &self.data
    }

    pub(crate) fn new_heur(
        name: SigName,
        description: Description,
        imports: Vec<Sha256Buff>,
    ) -> Signature {
        Self { base: SigBase { name, description }, data: SigData::Imports(imports) }
    }

    pub(crate) fn new_sha(name: SigName, description: Description, sha: Sha256Buff) -> Signature {
        Self { base: SigBase { name, description }, data: SigData::Sha(sha) }
    }

    pub(crate) fn description(&self) -> String {
        self.base.description.clone()
    }

    pub(crate) fn name(&self) -> String {
        self.base.name.clone()
    }
}

impl From<Signature> for DetectionReport {
    fn from(sig: Signature) -> Self {
        let cause = match sig.data {
            SigData::Sha(sha) => format!("Found sha: '{}'", convert_sha256_to_string(&sha)),
            SigData::Imports(imports) => format!("Found {} suspicious imports", imports.len()),
        };
        Self { name: sig.base.name, desc: sig.base.description, cause }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum SigData {
    Sha(Sha256Buff),
    Imports(Vec<Sha256Buff>),
}

impl SigData {
    fn from_yaml(yaml_sig_data: YamlSigData) -> Result<Self, SigSetError> {
        fn import_string_to_sha(s: &String) -> Sha256Buff {
            let import_sha = sha256_from_vec(s.to_lowercase().as_bytes().to_vec());
            #[cfg(debug_assertions)]
            log::debug!(
                "import: \"{} -- {}\"",
                s.to_lowercase(),
                convert_sha256_to_string(&import_sha)
            );
            import_sha
        }

        Ok(match yaml_sig_data {
            YamlSigData::Sha(sha) => Self::Sha(convert_string_to_sha256(sha.as_str())?),
            YamlSigData::Imports(imports) => {
                Self::Imports(imports.iter().map(|s| import_string_to_sha(s)).collect())
            },
        })
    }
}
