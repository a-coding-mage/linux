/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Corresponds to the C header guard _ASM_GENERIC_SWAB_H.
 */

/* Dependency: <asm/bitsperlong.h> supplies __BITS_PER_LONG. */

/*
 * 32 bit architectures typically (but not always) want to
 * set __SWAB_64_THRU_32__. In user space, this is only
 * valid if the compiler supports 64 bit data types.
 */

/*
 * C condition preserved: when __BITS_PER_LONG == 32, and either
 * (__GNUC__ is defined and __STRICT_ANSI__ is not defined) or
 * __KERNEL__ is defined, the C header defines __SWAB_64_THRU_32__.
 * The corresponding build-time Rust configuration is supplied externally.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
