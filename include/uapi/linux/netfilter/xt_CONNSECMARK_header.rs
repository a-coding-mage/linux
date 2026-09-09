/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: __u8 is supplied by the Linux types definitions.

pub const CONNSECMARK_SAVE: u32 = 1;
pub const CONNSECMARK_RESTORE: u32 = 2;

#[repr(C)]
pub struct xt_connsecmark_target_info {
    pub mode: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
