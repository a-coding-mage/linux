/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The C header guard __ASM_MIPS_BITSPERLONG_H is omitted in Rust.

/// Equivalent of the C macro `__BITS_PER_LONG`, supplied by the MIPS build configuration.
pub const __BITS_PER_LONG: usize = _MIPS_SZLONG;

// Dependency intent: declarations from <asm-generic/bitsperlong.h> are supplied externally.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
