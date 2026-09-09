/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Copyright (C) 1999  Niibe Yutaka
 * Copyright (C) 2000, 2001  Paolo Alberelli
 */

// The C header includes linux/compiler.h, linux/types.h, and asm-generic/swab.h.

#[inline]
pub fn __arch_swab32(mut x: u32) -> u32 {
    // Equivalent to the SH swap.b, swap.w, and swap.b instruction sequence.
    x = ((x & 0x00ff_00ff) << 8) | ((x & 0xff00_ff00) >> 8);
    x = x.rotate_left(16);
    x = ((x & 0x00ff_00ff) << 8) | ((x & 0xff00_ff00) >> 8);
    x
}

#[inline]
pub fn __arch_swab16(mut x: u16) -> u16 {
    // Equivalent to the SH swap.b instruction.
    x = (x << 8) | (x >> 8);
    x
}

#[inline]
pub fn __arch_swab64(val: u64) -> u64 {
    // C union layout is represented by extracting and recombining the two
    // 32-bit halves in the same order as the original implementation.
    let a = val as u32;
    let b = (val >> 32) as u32;
    let swapped_b = __arch_swab32(a);
    let swapped_a = __arch_swab32(b);
    ((swapped_a as u64) << 32) | (swapped_b as u64)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
