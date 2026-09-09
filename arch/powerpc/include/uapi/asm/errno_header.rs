/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The C header undefines EDEADLOCK before and after including
// <asm-generic/errno.h>; the included errno definitions are supplied by
// another translated dependency.

/// File locking deadlock error.
pub const EDEADLOCK: i32 = 58;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
