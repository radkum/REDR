use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum ExtractError {
    #[snafu(display("{error}"))]
    IoError { error: std::io::Error },
    #[snafu(display("{error}"))]
    OleError { error: ole::Error },
}

impl From<ole::Error> for ExtractError {
    fn from(error: ole::Error) -> Self {
        Self::OleError { error }
    }
}
