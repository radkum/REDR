use std::ffi::OsString;

use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum SignatureError {
    #[snafu(display("{error}"))]
    IoError { error: std::io::Error },
    #[snafu(display("Malset file must be divisible by 32"))]
    InvalidMalsetSizeError {},
    #[snafu(display("Can't convert: {os_string:?} to String"))]
    OsStringError { os_string: OsString },
}

impl From<std::io::Error> for SignatureError {
    fn from(arg: std::io::Error) -> Self {
        Self::IoError { error: arg }
    }
}
impl From<OsString> for SignatureError {
    fn from(arg: OsString) -> Self {
        Self::OsStringError { os_string: arg }
    }
}
