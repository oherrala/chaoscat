#![doc = include_str!("../README.md")]

use libc::{c_char, c_int, c_void};

mod utils;
use utils::*;

/// Intercept `open(path, flags)` syscall.
///
/// <https://man.openbsd.org/open.2>
#[no_mangle]
pub extern "C" fn open(path: *const c_char, flags: c_int) -> c_int {
    const SYMBOL: &str = "open";

    if is_enabled(SYMBOL) && do_chaos() {
        return return_errno(libc::EINTR).into();
    }

    let orig: unsafe extern "C" fn(*const c_char, c_int) -> c_int = load_sym(SYMBOL);

    unsafe { orig(path, flags) }
}

/// Intercept `read(fd, buf, len)` syscall.
///
/// This interceptor will randomly fail read() syscall by returning -1 and
/// setting errno to EINTR ("Interrupted system call").
///
/// Skips stdin, stdout and stderr so that basic interaction is possible.
///
/// <https://man.openbsd.org/read.2>
#[no_mangle]
pub extern "C" fn read(fd: c_int, buf: *const c_void, nbytes: libc::size_t) -> libc::ssize_t {
    const SYMBOL: &str = "read";

    // skips stdin, stdout and stderr
    if fd > 2 && is_enabled(SYMBOL) && do_chaos() {
        return return_errno(libc::EINTR).into();
    }

    let orig: unsafe extern "C" fn(c_int, *const c_void, libc::size_t) -> libc::ssize_t =
        load_sym(SYMBOL);

    unsafe { orig(fd, buf, nbytes) }
}

/// Intercept `write(fd, buf, len)` syscall.
///
/// This interceptor will randomly fail read() syscall by returning -1 and
/// setting errno to EINTR ("Interrupted system call").
///
/// Skips stdin, stdout and stderr so that basic interaction is possible.
///
/// <https://man.openbsd.org/write.2>
#[no_mangle]
pub extern "C" fn write(fd: c_int, buf: *const c_void, nbytes: libc::size_t) -> libc::ssize_t {
    const SYMBOL: &str = "write";

    // skips stdin, stdout and stderr
    if fd > 2 && is_enabled(SYMBOL) && do_chaos() {
        return return_errno(libc::EINTR).into();
    }

    let orig: unsafe extern "C" fn(c_int, *const c_void, libc::size_t) -> libc::ssize_t =
        load_sym(SYMBOL);

    unsafe { orig(fd, buf, nbytes) }
}

/// Intercept `socket(domain, type, protocol)` syscall.
///
/// This interceptor will randomly fail socket() syscall by returning -1 and
/// setting errno to ENOBUFS ("No buffer space available").
///
/// <https://man.openbsd.org/socket.2>

//  socket(int domain, int type, int protocol);
#[no_mangle]
pub extern "C" fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int {
    const SYMBOL: &str = "socket";

    if is_enabled(SYMBOL) && do_chaos() {
        return return_errno(libc::ENOBUFS).into();
    }

    let orig: unsafe extern "C" fn(c_int, c_int, c_int) -> c_int = load_sym(SYMBOL);

    unsafe { orig(domain, type_, protocol) }
}

/// Intercept `connect(socket, name, namelen)` syscall.
///
/// This interceptor will randomly fail socket() syscall by returning -1 and
/// setting errno to ENOBUFS ("No buffer space available").
///
/// <https://man.openbsd.org/socket.2>
#[no_mangle]
pub extern "C" fn connect(
    s: c_int,
    name: *const libc::sockaddr,
    namelen: libc::socklen_t,
) -> c_int {
    const SYMBOL: &str = "connect";

    if is_enabled(SYMBOL) && do_chaos() {
        return return_errno(libc::EINTR).into();
    }

    let orig: unsafe extern "C" fn(c_int, *const libc::sockaddr, libc::socklen_t) -> c_int =
        load_sym(SYMBOL);

    unsafe { orig(s, name, namelen) }
}

/// Intercept `sendto(socket, msg, len, flags, to, tolen)` syscall.
///
/// This interceptor will randomly fail sendto() syscall by returning -1 and
/// setting errno to ENOBUFS ("No buffer space available").
///
/// <https://man.openbsd.org/sendto.2>
#[no_mangle]
pub extern "C" fn sendto(
    s: c_int,
    msg: *const c_void,
    len: libc::size_t,
    flags: c_int,
    to: *const libc::sockaddr,
    tolen: libc::socklen_t,
) -> libc::ssize_t {
    const SYMBOL: &str = "sendto";

    if is_enabled(SYMBOL) && do_chaos() {
        return return_errno(libc::ENOBUFS).into();
    }

    let orig: unsafe extern "system" fn(
        c_int,
        *const c_void,
        libc::size_t,
        c_int,
        *const libc::sockaddr,
        libc::socklen_t,
    ) -> libc::ssize_t = load_sym(SYMBOL);

    unsafe { orig(s, msg, len, flags, to, tolen) }
}
