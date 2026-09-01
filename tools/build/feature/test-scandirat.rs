// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE before including <dirent.h> to expose scandirat.

use std::os::raw::{c_int, c_void};

extern "C" {
    fn scandirat(
        dirfd: c_int,
        dirp: *mut c_void,
        namelist: *mut c_void,
        filter: *mut c_void,
        compar: *mut c_void,
    ) -> c_int;
}

fn main() -> c_int {
    // expects non-NULL, arg3 is 'restrict' so "pointers" have to be different
    unsafe {
        scandirat(
            0,
            1usize as *mut c_void,
            2usize as *mut c_void,
            3usize as *mut c_void,
            4usize as *mut c_void,
        )
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
