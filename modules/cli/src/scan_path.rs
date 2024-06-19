use std::{collections::VecDeque, fs::File, path::PathBuf};

use common::redr;
use signatures::sig_store::SignatureStore;

pub(super) fn scan_path(target_path: &str, store_path: String) -> anyhow::Result<()> {
    log::debug!("scan_path: {}", target_path);
    let sig_store = signatures::deserialize_sig_store_from_path(store_path.as_str())?;

    let path = std::path::Path::new(target_path);

    if path.is_dir() {
        scan_dir(target_path, sig_store)?
    } else if path.is_file() {
        scan_file(target_path, sig_store)?
    } else {
        //other types are not supported
    }

    Ok(())
}

fn scan_file(file_path: &str, signature_store: SignatureStore) -> anyhow::Result<()> {
    log::debug!("scan_file: {}", file_path);
    let file = File::open(file_path)?;
    let file_info = redr::FileScanInfo::real_file(PathBuf::from(file_path));
    let file_to_scan = (redr::FileReader::from_file(file), file_info);

    let mut queue: VecDeque<(redr::FileReader, redr::FileScanInfo)> =
        VecDeque::from([file_to_scan]);
    scanner::scan_files(&mut queue, signature_store)?;

    Ok(())
}

fn scan_dir(dir_path: &str, signature_store: SignatureStore) -> anyhow::Result<()> {
    log::debug!("scan_dir: {}", dir_path);

    let mut queue = VecDeque::new();
    let paths = std::fs::read_dir(dir_path)?;
    for entry_res in paths {
        let entry = entry_res?;
        log::trace!("dir entry: {:?}", entry);

        if entry.file_type()?.is_file() {
            let path = entry.path();
            let file = File::open(&path)?;
            queue.push_back((
                redr::FileReader::from_file(file),
                redr::FileScanInfo::real_file(path.to_path_buf()),
            ));
        }
    }
    scanner::scan_files(&mut queue, signature_store)?;

    Ok(())
}
