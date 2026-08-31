// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <bfd.h>

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    fn bfd_demangle(abfd: *mut c_void, name: *const c_char, options: c_int) -> *mut c_char;
    fn printf(format: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let mut symbol = [0 as c_char; 4096];
    let init = b"FieldName__9ClassNameFd\0";
    let mut i = 0usize;

    while i < init.len() {
        symbol[i] = init[i] as c_char;
        i += 1;
    }

    let tmp: *mut c_char;

    tmp = unsafe { bfd_demangle(std::ptr::null_mut(), symbol.as_mut_ptr(), 0) };

    unsafe {
        printf(c"demangled symbol: {%s}\n".as_ptr(), tmp);
    }

    0
}
