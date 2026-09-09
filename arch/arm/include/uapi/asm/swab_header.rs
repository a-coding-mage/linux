/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  arch/arm/include/asm/byteorder.h
 *
 * ARM Endian-ness.  In little endian mode, the data bus is connected such
 * that byte accesses appear as:
 *  0 = d0...d7, 1 = d8...d15, 2 = d16...d23, 3 = d24...d31
 * and word accesses (data or instruction) appear as:
 *  d0...d31
 *
 * When in big endian mode, byte accesses appear as:
 *  0 = d24...d31, 1 = d16...d23, 2 = d8...d15, 3 = d0...d7
 * and word accesses (data or instruction) appear as:
 *  d0...d31
 */

// The C header defines __SWAB_64_THRU_32__ when strict ANSI mode is not in
// effect or when building the kernel.  That build-time condition is retained
// here as documentation; the symbol is supplied by the surrounding build.

// This declaration is active when !defined(__KERNEL__) || __LINUX_ARM_ARCH__ < 6.
// The C implementation's non-Thumb inline assembly is semantically equivalent
// to the expression below and is represented directly in Rust.
#[inline]
pub const fn __arch_swab32(mut x: u32) -> u32 {
    let mut t: u32;

    t = x ^ ((x << 16) | (x >> 16)); /* eor r1,r0,r0,ror #16 */

    x = (x << 24) | (x >> 8); /* mov r0,r0,ror #8 */
    t &= !0x00FF0000u32; /* bic r1,r1,#0x00FF0000 */
    x ^= t >> 8; /* eor r0,r0,r1,lsr #8 */

    x
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
