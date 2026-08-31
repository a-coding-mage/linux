/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_ulong, c_void};
use core::ptr;

pub const KSYM_NAME_LEN: usize = 512;

pub enum module {}

#[inline]
pub unsafe fn kallsyms_lookup(
    addr: c_ulong,
    symbolsize: *mut c_ulong,
    offset: *mut c_ulong,
    modname: *mut *mut c_char,
    namebuf: *mut c_char,
) -> *const c_char {
    let _ = addr;
    let _ = symbolsize;
    let _ = offset;
    let _ = modname;
    let _ = namebuf;

    ptr::null()
}

/* C conditional intent: #ifdef HAVE_BACKTRACE_SUPPORT */
#[cfg(feature = "have_backtrace_support")]
unsafe extern "C" {
    fn backtrace_symbols(buffer: *mut *mut c_void, size: i32) -> *mut *mut c_char;
    fn dprintf(fd: i32, format: *const c_char, ...) -> i32;
    fn free(ptr: *mut c_void);
}

/* C conditional intent: #ifdef HAVE_BACKTRACE_SUPPORT */
#[cfg(feature = "have_backtrace_support")]
#[inline]
pub unsafe fn print_ip_sym(loglvl: *const c_char, ip: c_ulong) {
    let mut name: *mut *mut c_char;
    let mut ip = ip;

    let _ = loglvl;

    name = unsafe { backtrace_symbols((&mut ip as *mut c_ulong).cast::<*mut c_void>(), 1) };

    unsafe {
        dprintf(1, c"%s\n".as_ptr(), *name);
        free(name.cast::<c_void>());
    }
}

/* C conditional intent: #else of #ifdef HAVE_BACKTRACE_SUPPORT */
#[cfg(not(feature = "have_backtrace_support"))]
#[inline]
pub unsafe fn print_ip_sym(loglvl: *const c_char, ip: c_ulong) {
    let _ = loglvl;
    let _ = ip;
}
