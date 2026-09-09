/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: `O_CLOEXEC` and `O_NONBLOCK` are supplied by <linux/fcntl.h>.

pub const EFD_SEMAPHORE: i32 = 1 << 0;
pub const EFD_CLOEXEC: i32 = O_CLOEXEC;
pub const EFD_NONBLOCK: i32 = O_NONBLOCK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
