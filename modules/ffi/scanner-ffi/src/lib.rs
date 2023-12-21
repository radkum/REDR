use common::{
    error::to_extern_error,
    ffi_trace,
    FfiWrapper,
};
use ffi_support::{
    call_with_result,
    ExternError,
    FfiStr,
};

#[function_name::named]
#[no_mangle]
pub extern "C" fn redr_scan_file(file_path: FfiStr, error: Option<&mut ExternError>) -> *mut u32 {
    // ) -> *mut ScanInfo {
    ffi_trace!("file_path: {:?}, error: {:?}", file_path.as_opt_str(), error);

    if error.is_none() {
        return core::ptr::null_mut();
    }
    let error = error.unwrap();

    call_with_result(error, || {
        scanner::match_checksum(file_path.into()).map(FfiWrapper::from).map_err(to_extern_error)
    })
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
