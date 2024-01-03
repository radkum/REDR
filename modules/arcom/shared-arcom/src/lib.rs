mod error;
use std::{
    collections::VecDeque,
    fs::File,
};

use common::redr;
pub use error::ExtractError;

pub trait FileExtractor {
    fn extract_files(
        &self,
        file: redr::FileReader,
        queue: &mut VecDeque<File>,
    ) -> Result<(), ExtractError>;
}
