/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: values supplied by <uapi/asm-generic/param.h>.

pub const HZ: i32 = CONFIG_HZ; // Internal kernel timer frequency
pub const USER_HZ: i32 = __USER_HZ; // some user interfaces are
pub const CLOCKS_PER_SEC: i32 = USER_HZ; // in "ticks" like times()

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
