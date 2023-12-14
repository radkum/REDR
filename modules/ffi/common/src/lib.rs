pub mod error;
mod ffi_wrapper;
pub mod trace;

pub use ffi_wrapper::FfiWrapper;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
