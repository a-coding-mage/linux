/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent: __s64 from <linux/types.h>.
#[repr(C)]
pub struct arch_vdso_time_data {
    pub tod_delta: i64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
