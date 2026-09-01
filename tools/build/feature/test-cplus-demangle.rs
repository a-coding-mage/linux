// SPDX-License-Identifier: GPL-2.0
use core::ffi::{c_char, c_int};

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn cplus_demangle(arg1: *const c_char, arg2: c_int) -> *mut c_char;
}

fn main() -> c_int {
    let mut symbol = [0 as c_char; 4096];
    let init = b"FieldName__9ClassNameFd\0";
    let mut i = 0usize;

    while i < init.len() {
        symbol[i] = init[i] as c_char;
        i += 1;
    }

    let tmp: *mut c_char;

    unsafe {
        tmp = cplus_demangle(symbol.as_ptr(), 0);

        printf(c"demangled symbol: {%s}\n".as_ptr(), tmp);
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
