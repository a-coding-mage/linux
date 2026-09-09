/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: declarations from <asm-generic/poll.h> are supplied externally.

pub const POLLWRNORM: i32 = POLLOUT;
pub const POLLWRBAND: i32 = 256;
pub const POLLMSG: i32 = 512;
pub const POLLREMOVE: i32 = 1024;
pub const POLLRDHUP: i32 = 2048;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
