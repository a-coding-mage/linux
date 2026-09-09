/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: declarations from <asm-generic/fcntl.h> are supplied by
// the corresponding Rust translation of that header.

pub const O_DIRECTORY: i32 = 1 << 14; // must be a directory
pub const O_NOFOLLOW: i32 = 1 << 15; // don't follow links
pub const O_DIRECT: i32 = 1 << 16; // direct disk access hint - currently ignored
pub const O_LARGEFILE: i32 = 1 << 17;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
