use std::{
    collections::VecDeque,
    io::Read,
};

use common::redr;
use shared_arcom::{
    ExtractError,
    FileExtractor,
};

pub struct ZipExtractor {}

impl FileExtractor for ZipExtractor {
    fn extract_files(
        &self,
        file: redr::FileReader,
        queue: &mut VecDeque<redr::FileReader>,
    ) -> Result<(), ExtractError> {
        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            //let file_reader = redr::FileReader::from_zip_file(file);
            let mut buffer = Vec::new();
            let size = file.read_to_end(&mut buffer)?;
            if file.size() as usize != size {
                log::error!("{}, {}", file.size(), size);
                todo!()
            }
            let reader = redr::FileReader::from_buff(std::io::Cursor::new(buffer));
            queue.push_front(reader);

            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };
            log::trace!("{:?}", outpath.as_path().as_os_str());
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
