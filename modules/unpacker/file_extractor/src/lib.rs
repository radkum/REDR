use std::{
    collections::VecDeque,
    fs::File,
};

pub trait FileExtractor {
    fn extract_files(&self, file: &mut File, queue: &mut VecDeque<File>);
}
