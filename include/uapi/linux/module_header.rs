/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Flags for sys_finit_module: */
pub const MODULE_INIT_IGNORE_MODVERSIONS: u32 = 1;
pub const MODULE_INIT_IGNORE_VERMAGIC: u32 = 2;
pub const MODULE_INIT_COMPRESSED_FILE: u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
