// SPDX-License-Identifier: GPL-2.0

// C dependencies: <linux/types.h>, "../../../util/tsc.h"

pub type u64 = core::ffi::c_ulonglong;

pub unsafe extern "C" fn rdtsc() -> u64 {
    let val: u64;

    /*
     * According to ARM DDI 0487F.c, from Armv8.0 to Armv8.5 inclusive, the
     * system counter is at least 56 bits wide; from Armv8.6, the counter
     * must be 64 bits wide.  So the system counter could be less than 64
     * bits wide and it is attributed with the flag 'cap_user_time_short'
     * is true.
     */
    core::arch::asm!("mrs {0}, cntvct_el0", out(reg) val, options(nomem, nostack, preserves_flags));

    val
}
