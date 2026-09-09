/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency equivalent of: #include <asm-generic/poll.h>

pub const POLLWRNORM: i32 = POLLOUT;
pub const POLLWRBAND: i32 = 256;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
