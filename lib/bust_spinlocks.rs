// SPDX-License-Identifier: GPL-2.0
/*
 * lib/bust_spinlocks.c
 *
 * Provides a minimal bust_spinlocks for architectures which don't
 * have one of their own.
 *
 * bust_spinlocks() clears any spinlocks which would prevent oops, die(), BUG()
 * and panic() information from reaching the user.
 */

// External symbols supplied by the kernel headers and other translation units.
use core::ffi::c_int;

unsafe extern "C" {
    static mut oops_in_progress: c_int;
    fn console_unblank();
    fn wake_up_klogd();
}

pub unsafe fn bust_spinlocks(yes: c_int) {
    if yes != 0 {
        oops_in_progress += 1;
    } else {
        console_unblank();
        oops_in_progress -= 1;
        if oops_in_progress == 0 {
            wake_up_klogd();
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
