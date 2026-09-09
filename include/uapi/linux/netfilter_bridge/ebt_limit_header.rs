/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency equivalent of <linux/types.h>: __u32 is u32 and unsigned long
// retains the target platform's C unsigned-long width.

pub const EBT_LIMIT_MATCH: &str = "limit";

/* timings are in milliseconds. */
pub const EBT_LIMIT_SCALE: u32 = 10000;

/* 1/10,000 sec period => max of 10,000/sec.  Min rate is then 429490
   seconds, or one every 59 hours. */

#[repr(C)]
pub struct ebt_limit_info {
    pub avg: u32,    /* Average secs between packets * scale */
    pub burst: u32,  /* Period multiplier for upper limit. */

    /* Used internally by the kernel */
    pub prev: core::ffi::c_ulong,
    pub credit: u32,
    pub credit_cap: u32,
    pub cost: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
