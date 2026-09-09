// SPDX-License-Identifier: GPL-2.0
/*
 * Strings for the various x86 power flags
 *
 * This file must not contain any executable code.
 */

use core::ffi::c_char;

pub static x86_power_flags: [*const c_char; 32] = [
    c"ts".as_ptr(),             /* temperature sensor */
    c"fid".as_ptr(),            /* frequency id control */
    c"vid".as_ptr(),            /* voltage id control */
    c"ttp".as_ptr(),            /* thermal trip */
    c"tm".as_ptr(),             /* hardware thermal control */
    c"stc".as_ptr(),            /* software thermal control */
    c"100mhzsteps".as_ptr(),    /* 100 MHz multiplier control */
    c"hwpstate".as_ptr(),       /* hardware P-state control */
    c"".as_ptr(),               /* tsc invariant mapped to constant_tsc */
    c"cpb".as_ptr(),            /* core performance boost */
    c"eff_freq_ro".as_ptr(),    /* Readonly aperf/mperf */
    c"proc_feedback".as_ptr(),  /* processor feedback interface */
    c"acc_power".as_ptr(),      /* accumulated power mechanism */
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
