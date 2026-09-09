/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* MONITOR/MWAIT enabled in Ring 3 */
pub const HWCAP2_RING3MWAIT: usize = 1usize << 0;

/* Kernel allows FSGSBASE instructions available in Ring 3 */
pub const HWCAP2_FSGSBASE: usize = 1usize << 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
