use std::{
    collections::VecDeque,
    fs::File,
};

use file_extractor::FileExtractor;

pub struct Ole {}

impl FileExtractor for Ole {
    fn extract_files(&self, file: &mut File, queue: &mut VecDeque<File>) {
        todo!();
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
