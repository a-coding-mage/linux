/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux uaccess layer.

#[inline]
unsafe fn __get_inst(i: *mut u32, p: *mut u32, user: bool) -> i32 {
    if user {
        get_user(i, p as *const u32)
    } else {
        get_kernel_nofault(i, p)
    }
}

#[inline]
unsafe fn __get_addr(a: *mut usize, p: *mut usize, user: bool) -> i32 {
    if user {
        get_user(a, p as *const usize)
    } else {
        get_kernel_nofault(a, p)
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
