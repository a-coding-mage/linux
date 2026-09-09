/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The C header includes <linux/const.h>; _BITUL(n) is represented here as
// an unsigned long-sized bit value.

pub const PVPANIC_PANICKED: usize = 1usize << 0;
pub const PVPANIC_CRASH_LOADED: usize = 1usize << 1;
pub const PVPANIC_SHUTDOWN: usize = 1usize << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
