// SPDX-License-Identifier: GPL-2.0
// C dependencies: <execinfo.h>, <stdio.h>

use std::ffi::c_void;

unsafe extern "C" {
    fn backtrace(buffer: *mut *mut c_void, size: i32) -> i32;
    fn backtrace_symbols_fd(buffer: *const *mut c_void, size: i32, fd: i32);
}

fn main() -> i32 {
    let mut backtrace_fns: [*mut c_void; 10] = [std::ptr::null_mut(); 10];
    let entries: i32;

    unsafe {
        entries = backtrace(backtrace_fns.as_mut_ptr(), 10);
        backtrace_symbols_fd(backtrace_fns.as_ptr(), entries, 1);
    }

    0
}
