use std::{
    collections::VecDeque,
    fs::File,
};

use common::redr;

pub(super) fn scan_file(file_path: &str, malset_path: &str) -> anyhow::Result<()> {
    log::debug!("{}", file_path);
    let signatures = signatures::get_malware_set(malset_path)?;
    let file = File::open(file_path)?;
    let file_to_scan = redr::FileReader::from_file(file);

    let mut queue: VecDeque<redr::FileReader> = VecDeque::from([file_to_scan]);
    scanner::scan_files(&mut queue, signatures)?;

    Ok(())
}
