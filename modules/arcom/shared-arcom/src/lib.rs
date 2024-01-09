mod error;
use std::collections::VecDeque;

use common::redr;
pub use error::ExtractError;

pub trait FileExtractor {
    fn extract_files(
        &self,
        file: redr::FileReader,
        queue: &mut VecDeque<redr::FileReader>,
    ) -> Result<(), ExtractError>;
}
