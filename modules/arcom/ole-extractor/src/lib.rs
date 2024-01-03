use std::{
    collections::VecDeque,
    fs::File,
};

use common::redr;
use shared_arcom::{
    ExtractError,
    FileExtractor,
};

pub struct OleExtractor {}

impl FileExtractor for OleExtractor {
    fn extract_files(
        &self,
        file: redr::FileReader,
        _queue: &mut VecDeque<File>,
    ) -> Result<(), ExtractError> {
        let binding = file.get_io_read();
        let parser = ole::Reader::new(&*binding)?;
        for entry in parser.iterate() {
            log::trace!("{}", entry.name());
        }
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
