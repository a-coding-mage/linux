// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <stdio.h>

unsafe extern "C" {
    fn puts(s: *const i8) -> i32;
}

fn main() -> i32 {
    unsafe { puts(c"hi".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
