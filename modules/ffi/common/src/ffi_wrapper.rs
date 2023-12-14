use ffi_support::IntoFfi;

#[repr(transparent)]
pub struct FfiWrapper<T>(*mut T);

unsafe impl<T> IntoFfi for FfiWrapper<T> {
    type Value = *mut T;

    fn ffi_default() -> Self::Value {
        core::ptr::null_mut()
    }

    fn into_ffi_value(self) -> Self::Value {
        self.0
    }
}

impl<T> From<T> for FfiWrapper<T> {
    fn from(item: T) -> Self {
        Self(Box::into_raw(Box::new(item)))
    }
}
