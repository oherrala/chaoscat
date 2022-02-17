use std::ffi::CString;

use libc::{c_int, c_void};

/// See if symbol is wanted to be broken in environment variable
/// `CHAOSCAT_OPTS`.
#[inline]
pub fn is_enabled(symbol: &str) -> bool {
    let opts = match std::env::var("CHAOSCAT_OPTS") {
        Ok(opts) => opts,
        Err(_) => return false,
    };

    opts.split(":").map(|s| s.trim()).any(|opt| opt == symbol)
}

/// If true, enable chaos. Failure should occur on 10% change.
#[inline]
pub fn do_chaos() -> bool {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf).unwrap();
    let i = f64::from(u32::from_be_bytes(buf));
    let limit = 0.1 * f64::from(u32::MAX);
    i < limit
}

/// Dynamically load symbol using [dlsym](https://man.openbsd.org/dlsym).
#[inline]
pub fn load_sym<T>(symbol: &str) -> T {
    let symbol = CString::new(symbol).unwrap();
    let ptr = unsafe { libc::dlsym(libc::RTLD_NEXT, symbol.as_ptr() as *const libc::c_char) };
    unsafe { std::ptr::read(&ptr as *const *mut c_void as *const _) }
}

/// Set `errno` and return with `-1` like properly failing syscall would do.
///
/// The return value is i8 so we can use `Into` trait to cast it to appropriate
/// sized integer.
#[inline]
pub fn return_errno(errno: c_int) -> i8 {
    #[cfg(target_os = "linux")]
    use libc::__errno_location as errno_location;
    #[cfg(target_os = "macos")]
    use libc::__error as errno_location;

    unsafe {
        *errno_location() = errno;
    }

    -1
}
