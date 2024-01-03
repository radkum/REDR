use std::{
    collections::{
        BTreeMap,
        VecDeque,
    },
    fs::File,
    io::{
        Read,
        Seek,
        SeekFrom,
    },
};

use common::redr;
use shared_arcom::ExtractError;
pub use shared_arcom::FileExtractor;
lazy_static::lazy_static! {
    static ref MAGIC_U64: BTreeMap<u64, ArchiveType> = [(0xe11ab1a1e011cfd0, ArchiveType::OLE)].into_iter().collect();
    static ref MAGIC_U32: BTreeMap<u32, ArchiveType> = [(0x04034b50, ArchiveType::ZIP)].into_iter().collect();
}

#[derive(Debug, Copy, Clone)]
enum ArchiveType {
    OLE,
    ZIP,
}

impl ArchiveType {
    fn check_x_bytes_magic(f: &mut File, nbytes: usize) -> Option<Self> {
        match nbytes {
            8 => {
                let mut bytes = [0u8; 8];
                if let Err(e) = f.read_exact(&mut bytes) {
                    log::info!("{e}");
                    return None;
                }

                let magic_u64: u64 = u64::from_le_bytes(bytes);
                //log::trace!("{:X}", magic_u64);
                MAGIC_U64.get(&magic_u64).map(|ft| ft.clone())
            },
            4 => {
                let mut bytes = [0u8; 4];
                if let Err(e) = f.read_exact(&mut bytes) {
                    log::info!("{e}");
                    return None;
                }

                let magic_u32: u32 = u32::from_le_bytes(bytes);
                MAGIC_U32.get(&magic_u32).map(|ft| ft.clone())
            },
            _ => todo!(),
        }
    }

    fn get_file_type(f: &mut File) -> Option<Self> {
        //write more sophisticated algorithm
        let ft_option = Self::check_x_bytes_magic(f, 8);
        if ft_option.is_some() {
            return ft_option;
        }
        Self::check_x_bytes_magic(f, 4)
    }

    fn get_file_extractor(&self) -> Box<dyn FileExtractor> {
        match self {
            ArchiveType::OLE => Box::new(ole_extractor::OleExtractor {}),
            ArchiveType::ZIP => todo!(),
        }
    }
}

pub fn unpack_file(mut file: File, queue: &mut VecDeque<File>) -> Result<(), ExtractError> {
    let file_type = ArchiveType::get_file_type(&mut file);

    if let Some(file_type) = file_type {
        log::info!("ArchiveType: {:?}", &file_type);
        let file_extractor = file_type.get_file_extractor();
        file.seek(SeekFrom::Start(0)).unwrap();
        file_extractor.extract_files(redr::FileReader::new(file), queue)
    } else {
        log::info!("Not known archive");
        Ok(())
    }
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
