/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the C header guard _UAPI_LINUX_TYPELIMITS_H.

pub const __KERNEL_INT_MAX: i32 = ((0u32).wrapping_not() >> 1) as i32;
pub const __KERNEL_INT_MIN: i32 = -__KERNEL_INT_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
