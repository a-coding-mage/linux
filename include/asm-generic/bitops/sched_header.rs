/* SPDX-License-Identifier: GPL-2.0 */

// `BITS_PER_LONG` is represented by the target pointer width in this
// translation.  The declarations below are supplied by other dependencies.

use std::os::raw::{c_int, c_ulong};

unsafe extern "C" {
    fn __ffs(word: c_ulong) -> c_int;
}

/*
 * Every architecture must define this function. It's the fastest
 * way of searching a 100-bit bitmap.  It's guaranteed that at least
 * one of the 100 bits is cleared.
 */
#[inline]
pub unsafe fn sched_find_first_bit(b: *const c_ulong) -> c_int {
    #[cfg(target_pointer_width = "64")]
    {
        if *b.add(0) != 0 {
            return __ffs(*b.add(0));
        }
        return __ffs(*b.add(1)) + 64;
    }

    #[cfg(target_pointer_width = "32")]
    {
        if *b.add(0) != 0 {
            return __ffs(*b.add(0));
        }
        if *b.add(1) != 0 {
            return __ffs(*b.add(1)) + 32;
        }
        if *b.add(2) != 0 {
            return __ffs(*b.add(2)) + 64;
        }
        return __ffs(*b.add(3)) + 96;
    }

    #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
    compile_error!("BITS_PER_LONG not defined");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
