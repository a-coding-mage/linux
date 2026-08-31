// SPDX-License-Identifier: GPL-2.0
// C++ dependencies in the original source:
// #include <stdio.h>
// #include <stdlib.h>
// #include <cxxabi.h>

use std::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;

    #[link_name = "__cxa_demangle"]
    fn __cxa_demangle(
        mangled_name: *const c_char,
        output_buffer: *mut c_char,
        length: *mut usize,
        status: *mut c_int,
    ) -> *mut c_char;
}

fn main() {
    unsafe {
        let mut len: usize = 256;
        let mut output: *mut c_char = malloc(len) as *mut c_char;
        let mut status: c_int;

        output = __cxa_demangle(
            c"FieldName__9ClassNameFd".as_ptr(),
            output,
            &mut len,
            &mut status,
        );

        printf(c"demangled symbol: {%s}\n".as_ptr(), output);

        return;
    }
}
