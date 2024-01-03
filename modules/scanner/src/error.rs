use signatures::signatures;
use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum ScanError {
    #[snafu(display("{error}"))]
    IoError { error: std::io::Error },
    #[snafu(display("{error}"))]
    SignaturesError { error: signatures::SignatureError },
    #[snafu(display("{error}"))]
    ExtractError { error: shared_arcom::ExtractError },
}

impl From<std::io::Error> for ScanError {
    fn from(arg: std::io::Error) -> Self {
        Self::IoError { error: arg }
    }
}

impl From<signatures::SignatureError> for ScanError {
    fn from(arg: signatures::SignatureError) -> Self {
        Self::SignaturesError { error: arg }
    }
}

impl From<shared_arcom::ExtractError> for ScanError {
    fn from(arg: shared_arcom::ExtractError) -> Self {
        Self::ExtractError { error: arg }
    }
}
