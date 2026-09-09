/* SPDX-License-Identifier: GPL-2.0 */
/*
 * bitops.h: Bit string operations on the V9.
 *
 * Copyright 1996, 1997 David S. Miller (davem@caip.rutgers.edu)
 */

// C header guard: _SPARC64_BITOPS_H
// This header is intended to be included only through <linux/bitops.h>.

// Dependencies supplied by the surrounding translation unit:
// <linux/compiler.h>, <asm/byteorder.h>, <asm/barrier.h>

extern "C" {
    pub fn test_and_set_bit(nr: u64, addr: *mut u64) -> i32;
    pub fn test_and_clear_bit(nr: u64, addr: *mut u64) -> i32;
    pub fn test_and_change_bit(nr: u64, addr: *mut u64) -> i32;
    pub fn set_bit(nr: u64, addr: *mut u64);
    pub fn clear_bit(nr: u64, addr: *mut u64);
    pub fn change_bit(nr: u64, addr: *mut u64);

    // C __attribute_const__ declarations.
    pub fn fls(word: u32) -> i32;
    pub fn __fls(word: u64) -> i32;
}

// <asm-generic/bitops/non-atomic.h>
// <asm-generic/bitops/fls64.h>

// The following declarations and generic facilities are present when
// __KERNEL__ is defined.
#[cfg(feature = "kernel")]
extern "C" {
    pub fn ffs(x: i32) -> i32;
    pub fn __ffs(word: u64) -> u64;

    pub fn __arch_hweight64(w: u64) -> u64;
    pub fn __arch_hweight32(w: u32) -> u32;
    pub fn __arch_hweight16(w: u32) -> u32;
    pub fn __arch_hweight8(w: u32) -> u32;
}

// <asm-generic/bitops/ffz.h>
// <asm-generic/bitops/sched.h>
// <asm-generic/bitops/const_hweight.h>
// <asm-generic/bitops/lock.h>

// When __KERNEL__ is defined, also include:
// <asm-generic/bitops/le.h>
// <asm-generic/bitops/ext2-atomic-setbit.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
