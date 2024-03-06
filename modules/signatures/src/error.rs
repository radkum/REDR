use std::ffi::OsString;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MsetError {
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Can't convert to String")]
    OsStringError(),
    #[error("BincodeDeserializeError: {0}")]
    BincodeDeserializeError(#[from] bincode::error::DecodeError),
    #[error("BincodeSerializeError: {0}")]
    BincodeSerializeError(#[from] bincode::error::EncodeError),
    #[error("Verification of file magic failed. Found '{current}'")]
    IncorrectMagicError { current: String },
    #[error("Verification of file checksum failed. Expected '{expected}' but found '{current}'")]
    MsetChecksumError { current: String, expected: String },
    #[error("Incorrect file size. Size: '{size}'")]
    IncorrectFileSizeError { size: u64 },
    #[error("Incorrect hs database size. Size: '{size}'")]
    IncorrectSignatureSizeError { size: u32 },
}

// #[derive(Snafu, Debug)]
// pub enum SignatureError {
//     #[snafu(display("{error}"))]
//     IoError { error: std::io::Error },
//     #[snafu(display("Malset file must be divisible by 32"))]
//     InvalidMalsetSizeError {},
//     #[snafu(display("Can't convert: {os_string:?} to String"))]
//     OsStringError { os_string: OsString },
// }
//
// impl From<std::io::Error> for SignatureError {
//     fn from(arg: std::io::Error) -> Self {
//         Self::IoError { error: arg }
//     }
// }
impl From<OsString> for MsetError {
    fn from(arg: OsString) -> Self {
        Self::OsStringError()
    }
}
