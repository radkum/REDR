#[macro_export]
macro_rules! ffi_trace {
    ($msg:expr) => {{
        log::trace!(
            concat!("{:?} | ", function_name!(), ": {}"),
            std::thread::current().id(),
            $msg
        );
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        log::trace!(
            concat!("{:?} | ", function_name!(), ": ", $fmt),
            std::thread::current().id(),
            $($arg)*
        );
    }};
}
