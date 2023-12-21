pub mod error;

use std::{
    collections::VecDeque,
    fs::File,
    io::{
        Seek,
        SeekFrom::Start,
    },
};

use signatures::signatures::Signatures;

const MAX_FILE_TO_SCAN: usize = 0x100;
use crate::error::ScanError;

pub fn scan_files(
    files_queue: &mut VecDeque<File>,
    signatures: Signatures,
) -> Result<(), ScanError> {
    for i in 1..MAX_FILE_TO_SCAN + 1 {
        if let Some(mut f) = files_queue.pop_front() {
            log::debug!("Start scanning {i} file");
            let _file_info = signatures.eval_file(&mut f)?;
            //do_action(_file_info)
            f.seek(Start(0))?;
            unpacker::unpack_file(&mut f, files_queue);
        } else {
            log::info!("No more files to scan");
            break;
        }
    }
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
