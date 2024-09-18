#![deny(warnings)]

#![no_std]

extern crate rlibc;

pub use rlibc::*;

use core::ffi::{c_int, c_char, c_float, c_ulonglong};

#[cfg(any(target_os="dos", docsrs))]
#[no_mangle]
pub extern "C" fn _chkstk() { }

#[cfg(any(target_os="dos", docsrs))]
#[no_mangle]
#[used]
pub static mut _fltused: c_int = 0;

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> usize {
    let mut n = s;
    while *n != 0 {
        n = n.offset(1);
    }
    n.offset_from(s) as usize
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn fminf(x: c_float, y: c_float) -> c_float {
    if x.is_nan() { return y; }
    if y.is_nan() { return x; }
    if x < y { x } else { y }
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn _aulldiv(a: c_ulonglong, b: c_ulonglong) -> c_ulonglong {
    a / b
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn _aullrem(a: c_ulonglong, b: c_ulonglong) -> c_ulonglong {
    a % b
}
