/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * In order to keep safe and avoid regression, only unify uapi
 * bitsperlong.h for some archs which are using newer toolchains
 * that have the definitions of __CHAR_BIT__ and __SIZEOF_LONG__.
 * See the following link for more info:
 * https://lore.kernel.org/linux-arch/b9624545-2c80-49a1-ac3c-39264a591f7b@app.fastmail.com/
 *
 * The C preprocessor definitions __CHAR_BIT__ and __SIZEOF_LONG__ are
 * build-time/compiler-provided values. Rust has no direct equivalent in
 * this isolated header, so the generic fallback value is preserved here.
 */
pub const __BITS_PER_LONG: usize = 32;

pub const __BITS_PER_LONG_LONG: usize = 64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
