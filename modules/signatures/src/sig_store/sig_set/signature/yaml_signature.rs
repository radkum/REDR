use serde::{Deserialize, Serialize};

use super::SigBase;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct YamlSignature {
    #[serde(flatten)]
    pub(crate) base: SigBase,
    #[serde(flatten)]
    pub(crate) data: YamlSigData,
}

#[derive(Debug, Serialize, Deserialize)]
//#[serde(untagged)]
pub(crate) enum YamlSigData {
    #[serde(rename = "sha256")]
    Sha(String),
    #[serde(rename = "imports")]
    Imports(Vec<String>),
}
