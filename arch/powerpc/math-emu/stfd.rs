// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to linux/types.h, linux/errno.h, and
// linux/uaccess.h are supplied externally.

use core::ffi::c_void;

const EFAULT: i32 = 14;

unsafe extern "C" {
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
}

pub unsafe fn stfd(fr_s: *mut c_void, ea: *mut c_void) -> i32 {
    if copy_to_user(ea, fr_s as *const c_void, core::mem::size_of::<f64>()) != 0 {
        return -EFAULT;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
