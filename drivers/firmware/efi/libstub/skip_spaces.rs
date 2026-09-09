// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel environment:
// linux/ctype.h, linux/string.h, linux/types.h

unsafe extern "C" {
    fn isspace(c: core::ffi::c_int) -> core::ffi::c_int;
}

pub unsafe fn skip_spaces(mut str_: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    while isspace(*str_ as core::ffi::c_int) != 0 {
        str_ = str_.add(1);
    }
    str_ as *mut core::ffi::c_char
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
