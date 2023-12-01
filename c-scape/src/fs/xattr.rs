//! Extended attributes.

use crate::{convert_res, READ_BUFFER};
use core::cmp::min;
use core::ffi::CStr;
use core::ptr::copy_nonoverlapping;
use core::slice;
use libc::{c_char, c_int, c_void, size_t, ssize_t};
use rustix::fd::BorrowedFd;
use rustix::fs::XattrFlags;

#[no_mangle]
unsafe extern "C" fn getxattr(
    path: *const c_char,
    name: *const c_char,
    value: *mut c_void,
    len: size_t,
) -> ssize_t {
    libc!(libc::getxattr(path, name, value, len));

    let path = CStr::from_ptr(path);
    let name = CStr::from_ptr(name);
    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    match convert_res(rustix::fs::getxattr(
        path,
        name,
        &mut READ_BUFFER[..min(len, READ_BUFFER.len())],
    )) {
        Some(size) => {
            // If `size` is 0, `value` could be null.
            if size != 0 {
                copy_nonoverlapping(READ_BUFFER.as_ptr(), value.cast(), size);
            }
            size as ssize_t
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn lgetxattr(
    path: *const c_char,
    name: *const c_char,
    value: *mut c_void,
    len: size_t,
) -> ssize_t {
    libc!(libc::lgetxattr(path, name, value, len));

    let path = CStr::from_ptr(path);
    let name = CStr::from_ptr(name);
    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    match convert_res(rustix::fs::lgetxattr(
        path,
        name,
        &mut READ_BUFFER[..min(len, READ_BUFFER.len())],
    )) {
        Some(size) => {
            // If `size` is 0, `value` could be null.
            if size != 0 {
                copy_nonoverlapping(READ_BUFFER.as_ptr(), value.cast(), size);
            }
            size as ssize_t
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn fgetxattr(
    fd: c_int,
    name: *const c_char,
    value: *mut c_void,
    len: size_t,
) -> ssize_t {
    libc!(libc::fgetxattr(fd, name, value, len));

    let fd = BorrowedFd::borrow_raw(fd);
    let name = CStr::from_ptr(name);
    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    match convert_res(rustix::fs::fgetxattr(
        fd,
        name,
        &mut READ_BUFFER[..min(len, READ_BUFFER.len())],
    )) {
        Some(size) => {
            // If `size` is 0, `value` could be null.
            if size != 0 {
                copy_nonoverlapping(READ_BUFFER.as_ptr(), value.cast(), size);
            }
            size as ssize_t
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn setxattr(
    path: *const c_char,
    name: *const c_char,
    value: *const c_void,
    len: size_t,
    flags: c_int,
) -> c_int {
    libc!(libc::setxattr(path, name, value, len, flags));

    let path = CStr::from_ptr(path);
    let name = CStr::from_ptr(name);
    let value = slice::from_raw_parts(value.cast(), len);
    let flags = XattrFlags::from_bits(flags as _).unwrap();
    match convert_res(rustix::fs::setxattr(path, name, value, flags)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn lsetxattr(
    path: *const c_char,
    name: *const c_char,
    value: *const c_void,
    len: size_t,
    flags: c_int,
) -> c_int {
    libc!(libc::lsetxattr(path, name, value, len, flags));

    let path = CStr::from_ptr(path);
    let name = CStr::from_ptr(name);
    let value = slice::from_raw_parts(value.cast(), len);
    let flags = XattrFlags::from_bits(flags as _).unwrap();
    match convert_res(rustix::fs::lsetxattr(path, name, value, flags)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn fsetxattr(
    fd: c_int,
    name: *const c_char,
    value: *const c_void,
    len: size_t,
    flags: c_int,
) -> c_int {
    libc!(libc::fsetxattr(fd, name, value, len, flags));

    let fd = BorrowedFd::borrow_raw(fd);
    let name = CStr::from_ptr(name);
    let value = slice::from_raw_parts(value.cast(), len);
    let flags = XattrFlags::from_bits(flags as _).unwrap();
    match convert_res(rustix::fs::fsetxattr(fd, name, value, flags)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn listxattr(path: *const c_char, list: *mut c_char, len: size_t) -> ssize_t {
    libc!(libc::listxattr(path, list, len));

    let path = CStr::from_ptr(path);
    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    match convert_res(rustix::fs::listxattr(
        path,
        &mut READ_BUFFER[..min(len, READ_BUFFER.len())],
    )) {
        Some(size) => {
            // If `size` is 0, `value` could be null.
            if size != 0 {
                copy_nonoverlapping(READ_BUFFER.as_ptr(), list.cast(), size);
            }
            size as ssize_t
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn llistxattr(path: *const c_char, list: *mut c_char, len: size_t) -> ssize_t {
    libc!(libc::llistxattr(path, list, len));

    let path = CStr::from_ptr(path);
    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    match convert_res(rustix::fs::llistxattr(
        path,
        &mut READ_BUFFER[..min(len, READ_BUFFER.len())],
    )) {
        Some(size) => {
            // If `size` is 0, `value` could be null.
            if size != 0 {
                copy_nonoverlapping(READ_BUFFER.as_ptr(), list.cast(), size);
            }
            size as ssize_t
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn flistxattr(fd: c_int, list: *mut c_char, len: size_t) -> ssize_t {
    libc!(libc::flistxattr(fd, list, len));

    let fd = BorrowedFd::borrow_raw(fd);
    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    match convert_res(rustix::fs::flistxattr(
        fd,
        &mut READ_BUFFER[..min(len, READ_BUFFER.len())],
    )) {
        Some(size) => {
            // If `size` is 0, `value` could be null.
            if size != 0 {
                copy_nonoverlapping(READ_BUFFER.as_ptr(), list.cast(), size);
            }
            size as ssize_t
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn removexattr(path: *const c_char, name: *const c_char) -> c_int {
    libc!(libc::removexattr(path, name));

    let path = CStr::from_ptr(path);
    let name = CStr::from_ptr(name);
    match convert_res(rustix::fs::removexattr(path, name)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn lremovexattr(path: *const c_char, name: *const c_char) -> c_int {
    libc!(libc::lremovexattr(path, name));

    let path = CStr::from_ptr(path);
    let name = CStr::from_ptr(name);
    match convert_res(rustix::fs::lremovexattr(path, name)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn fremovexattr(fd: c_int, name: *const c_char) -> c_int {
    libc!(libc::fremovexattr(fd, name));

    let fd = BorrowedFd::borrow_raw(fd);
    let name = CStr::from_ptr(name);
    match convert_res(rustix::fs::fremovexattr(fd, name)) {
        Some(()) => 0,
        None => -1,
    }
}
