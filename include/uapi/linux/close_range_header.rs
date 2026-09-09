/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Unshare the file descriptor table before closing file descriptors. */
pub const CLOSE_RANGE_UNSHARE: u32 = 1u32 << 1;

/* Set the FD_CLOEXEC bit instead of closing the file descriptor. */
pub const CLOSE_RANGE_CLOEXEC: u32 = 1u32 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
