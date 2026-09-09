/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency equivalent of: #include <linux/types.h>

#[repr(C)]
pub struct xt_classify_target_info {
    pub priority: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
