use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum SignaturesError {
    #[snafu(display("{file}"))]
    FileNotExistsError { file: String },
}

#[derive(thiserror::Error, Debug)]
pub enum ScanError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("error: {0}")]
    SignaturesError(#[from] SignaturesError),
    // #[error("Some '{0}'")]
    // Y(String),
}
