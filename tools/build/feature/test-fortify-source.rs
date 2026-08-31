// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <stdio.h>

unsafe extern "C" {
    fn puts(s: *const core::ffi::c_char) -> core::ffi::c_int;
}

fn main() -> core::ffi::c_int {
    unsafe { puts(c"hi".as_ptr()) }
}
