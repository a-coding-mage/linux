/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const MAP_32BIT: i32 = 0x40; /* only give out 32bit addresses */
pub const MAP_ABOVE4G: i32 = 0x80; /* only map above 4GB */

// Dependency intent: declarations from <asm-generic/mman.h> are supplied by
// the corresponding translated dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
