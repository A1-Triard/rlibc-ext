#![deny(warnings)]

#![no_std]

extern crate rlibc;

pub use rlibc::*;

#[cfg(any(target_os="dos", docsrs))]
use core::arch::asm;
#[cfg(any(target_os="dos", docsrs))]
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

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn _alldiv(a: c_longlong, b: c_longlong) -> c_longlong {
    a / b
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn _allrem(a: c_longlong, b: c_longlong) -> c_longlong {
    a % b
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_load(src: *const usize, _model: c_int) -> usize {
    asm!("cli");
    let dest = *src;
    asm!("sti", "nop");
    dest
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_store(dest: *mut usize, src: usize, _model: c_int) {
    asm!("cli");
    *dest = src;
    asm!("sti", "nop");
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_load_1(src: *const u8, _model: c_int) -> u8 {
    asm!("cli");
    let dest = *src;
    asm!("sti", "nop");
    dest
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_store_1(dest: *mut u8, src: u8, _model: c_int) {
    asm!("cli");
    *dest = src;
    asm!("sti", "nop");
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_load_2(src: *const u16, _model: c_int) -> u16 {
    asm!("cli");
    let dest = *src;
    asm!("sti", "nop");
    dest
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_store_2(dest: *mut u16, src: u16, _model: c_int) {
    asm!("cli");
    *dest = src;
    asm!("sti", "nop");
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_load_4(src: *const u32, _model: c_int) -> u32 {
    asm!("cli");
    let dest = *src;
    asm!("sti", "nop");
    dest
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_store_4(dest: *mut u32, src: u32, _model: c_int) {
    asm!("cli");
    *dest = src;
    asm!("sti", "nop");
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_load_8(src: *const u64, _model: c_int) -> u64 {
    asm!("cli");
    let dest = *src;
    asm!("sti", "nop");
    dest
}

#[cfg(any(target_os="dos", docsrs))]
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn __atomic_store_8(dest: *mut u64, src: u64, _model: c_int) {
    asm!("cli");
    *dest = src;
    asm!("sti", "nop");
}
