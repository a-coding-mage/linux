// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <stdio.h>

unsafe extern "C" {
    fn puts(s: *const i8) -> i32;
}

fn main() -> i32 {
    unsafe { puts(c"hi".as_ptr()) }
}
