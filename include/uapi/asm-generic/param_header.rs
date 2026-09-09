/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C preprocessor conditional definitions are represented here as constants;
// downstream configuration may provide equivalent overrides.
pub const __USER_HZ: i32 = 100;
pub const HZ: i32 = __USER_HZ;
pub const EXEC_PAGESIZE: i32 = 4096;
pub const NOGROUP: i32 = -1;
pub const MAXHOSTNAMELEN: i32 = 64; // max length of hostname

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
