/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __ASM_CSKY_BITOPS_H

// Dependencies supplied by the surrounding kernel translation:
// <linux/compiler.h>, <asm/barrier.h>, and the asm-generic bitops headers.

/*
 * asm-generic/bitops/ffs.h
 */
#[inline]
pub const fn ffs(mut x: i32) -> i32 {
    if x == 0 {
        return 0;
    }

    // C implementation uses C-SKY: brev, ff1, addi 1.
    x = x.trailing_zeros() as i32 + 1;
    x
}

/*
 * asm-generic/bitops/__ffs.h
 */
#[inline]
pub const fn __ffs(x: usize) -> usize {
    // C implementation uses C-SKY: brev followed by ff1.
    x.trailing_zeros() as usize
}

/*
 * asm-generic/bitops/fls.h
 */
#[inline]
pub const fn fls(x: u32) -> i32 {
    // C implementation uses C-SKY ff1; unsigned int is 32 bits here.
    32 - x.leading_zeros() as i32
}

/*
 * asm-generic/bitops/__fls.h
 */
#[inline]
pub const fn __fls(x: usize) -> usize {
    fls(x as u32) as usize - 1
}

// Included declarations from asm-generic/bitops/ffz.h and fls64.h are
// provided by the surrounding translation.

// This header requires inclusion through <linux/bitops.h> (_LINUX_BITOPS_H).

// Included declarations from asm-generic/bitops/sched.h, hweight.h, lock.h,
// atomic.h, non-atomic.h, le.h, and ext2-atomic.h are provided by the
// surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
