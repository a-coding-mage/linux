/* SPDX-License-Identifier: GPL-2.0 */

/* The declarations below are available when building the kernel (__KERNEL__). */

/* Grossly misnamed. */
#[repr(i32)]
pub enum die_val {
    DIE_OOPS = 1,
    DIE_IABR_MATCH,
    DIE_DABR_MATCH,
    DIE_BPT,
    DIE_SSTEP,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
