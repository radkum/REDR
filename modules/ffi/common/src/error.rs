use ffi_support::{
    ErrorCode,
    ExternError,
};

pub trait IntoExternError: core::fmt::Debug {
    fn get_message(&self) -> String {
        format!("{:?}", self)
    }
    fn get_code(&self) -> ErrorCode;
}

impl IntoExternError for scanner::ScanError {
    fn get_code(&self) -> ErrorCode {
        ErrorCode::new(error_code::SCAN_ERROR)
    }
}

pub fn to_extern_error(err: impl IntoExternError) -> ExternError {
    ExternError::new_error(err.get_code(), err.get_message())
}

mod error_code {
    pub const SCAN_ERROR: i32 = 1;
}
