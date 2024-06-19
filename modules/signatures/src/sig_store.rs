use std::{
    io::{Seek, SeekFrom},
    mem::size_of,
};

use common::{detection::DetectionReport, redr};
use serde::{Deserialize, Serialize};

pub mod sig_set;

use sig_set::{
    heuristic_set::HeurSet,
    sha_set::ShaSet,
    signature::{yaml_signature::YamlSignature, SigData, Signature},
    sigset_deserializer::SigSetDeserializer,
    sigset_serializer::BIN_CONFIG,
    SigSetTrait,
};

use crate::SigSetError;

#[derive(Debug, Serialize, Deserialize)]
struct StoreHeader {
    magic: u32,
    elem_count: u32,
}

impl StoreHeader {
    pub(crate) const HEADER_SIZE: usize = size_of::<StoreHeader>();
    const STORE_MAGIC_U32: u32 = 0x5445354D;

    //M5ET
    //const SHASET_MAGIC: [u8; 4] = [0x4D, 0x35, 0x45, 0x54]; //M5ET

    pub fn new(elem_count: u32) -> Self {
        Self { magic: Self::STORE_MAGIC_U32, elem_count }
    }

    pub(crate) fn verify_magic(&self) -> Result<(), SigSetError> {
        if Self::STORE_MAGIC_U32 != self.magic {
            return Err(SigSetError::IncorrectMagicError {
                current: String::from_utf8_lossy(&self.magic.to_le_bytes()).into(),
            });
        }
        Ok(())
    }

    #[inline]
    pub fn elem_count(&self) -> u32 {
        self.elem_count
    }
}

pub struct SignatureStore {
    sigset_vec: Vec<Box<dyn SigSetTrait>>,
}

impl SignatureStore {
    pub(crate) fn new(sigset_vec: Vec<Box<dyn SigSetTrait>>) -> Self {
        Self { sigset_vec }
    }

    pub fn eval_file(
        &self,
        file: &mut redr::FileReader,
    ) -> Result<Option<DetectionReport>, SigSetError> {
        for sig_set in self.sigset_vec.iter() {
            file.seek(SeekFrom::Start(0))?;
            if let Some(detection) = sig_set.eval_file(file)? {
                return Ok(Some(detection));
            }
        }
        Ok(None)
    }

    fn get_signatures(
        path_to_dir: &std::path::Path,
        signatures: &mut Vec<Signature>,
    ) -> Result<(), SigSetError> {
        let paths = std::fs::read_dir(path_to_dir)?;

        for entry_res in paths {
            let entry = entry_res?;
            //log::trace!("path: {:?}", &path);
            if entry.file_type()?.is_file() {
                let f = std::fs::File::open(entry.path())?;
                let yaml_sig: YamlSignature = serde_yaml::from_reader(&f).unwrap();

                signatures.push(Signature::from_yaml(yaml_sig)?);
            } else if entry.file_type()?.is_dir() {
                Self::get_signatures(entry.path().as_path(), signatures)?
            }
        }
        Ok(())
    }

    pub(crate) fn from_yaml_signatures(path_to_dir: &str) -> Result<Self, SigSetError> {
        let mut sha_set = ShaSet::new_empty();
        let mut heur_set = HeurSet::new_empty();

        let mut signatures = vec![];
        let path = std::path::Path::new(path_to_dir);
        Self::get_signatures(path, &mut signatures)?;

        let mut sig_id = 0;
        for sig in signatures {
            match sig.sig_data() {
                SigData::Sha(..) => sha_set.append_signature(sig_id, sig),
                SigData::Imports(..) => heur_set.append_signature(sig_id, sig),
            }
            sig_id += 1;
        }

        log::info!("sig store size: {}", sig_id);
        Ok(Self::new(vec![Box::new(sha_set), Box::new(heur_set)]))
    }

    pub(crate) fn deserialize<R: std::io::Read>(mut io_reader: R) -> Result<Self, SigSetError> {
        let mut data = vec![];
        let _size = io_reader.read_to_end(&mut data)?;
        Self::deserialize_vec(&mut data)
    }

    pub(crate) fn deserialize_vec(data: &mut Vec<u8>) -> Result<Self, SigSetError> {
        if data.len() < StoreHeader::HEADER_SIZE {
            return Err(SigSetError::IncorrectFileSizeError { size: data.len() as u64 });
        }

        let store_header_data = data.drain(..StoreHeader::HEADER_SIZE).collect::<Vec<u8>>();
        let store_header: StoreHeader =
            bincode::serde::decode_from_slice(&store_header_data, BIN_CONFIG)?.0;

        store_header.verify_magic()?;

        let mut sigset_vec = vec![];
        for _ in 0..store_header.elem_count() {
            let des = SigSetDeserializer::new_with_buffer(data)?;
            sigset_vec.push(des.get_set_box()?);
        }

        Ok(Self::new(sigset_vec))
    }

    pub(crate) fn serialize<W: std::io::Write>(&self, out: &mut W) -> Result<usize, SigSetError> {
        let store_header = StoreHeader::new(self.sigset_vec.len() as u32);
        let header_vec = bincode::serde::encode_to_vec(&store_header, BIN_CONFIG)?;
        out.write_all(&header_vec)?;

        let mut serialized_sigs = 0;
        for sigset in self.sigset_vec.iter() {
            let size = sigset.to_set_serializer().serialize(out)?;
            serialized_sigs += size;
        }

        Ok(serialized_sigs)
    }
}
