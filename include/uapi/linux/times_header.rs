/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Translated from <linux/types.h>; __kernel_clock_t is supplied externally. */
#[repr(C)]
pub struct tms {
    pub tms_utime: __kernel_clock_t,
    pub tms_stime: __kernel_clock_t,
    pub tms_cutime: __kernel_clock_t,
    pub tms_cstime: __kernel_clock_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
