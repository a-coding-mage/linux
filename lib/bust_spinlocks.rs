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

// Dependencies: linux/printk.h, linux/console.h.

use core::ptr::{addr_of_mut, read_volatile, write_volatile};
use kernel::bindings::{oops_in_progress, wake_up_klogd};
use kernel::ffi::c_int;

extern "C" {
    fn console_unblank();
}

/// Enters (`yes` != 0) or leaves an oops, waking klogd on the final exit.
#[no_mangle]
pub extern "C" fn bust_spinlocks(yes: c_int) {
    // SAFETY: oops_in_progress is an always-valid global, updated with the
    // same plain read-modify-write (wrapping, -fno-strict-overflow) as C.
    unsafe {
        let counter = addr_of_mut!(oops_in_progress);
        if yes != 0 {
            write_volatile(counter, read_volatile(counter).wrapping_add(1));
        } else {
            console_unblank();
            let value = read_volatile(counter).wrapping_sub(1);
            write_volatile(counter, value);
            if value == 0 {
                wake_up_klogd();
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
