/* SPDX-License-Identifier: GPL-2.0 */

/*
 * For the benefit of those who are trying to port Linux to another
 * architecture, here are some C-language equivalents.  They should
 * generate reasonable code, so take a look at what your compiler spits
 * out before rolling your own buggy implementation in assembly language.
 *
 * C language equivalents written by Theodore Ts'o, 9/26/92
 */

// Dependencies supplied by the surrounding Linux/Rust translation:
// linux/irqflags.h
// linux/compiler.h
// asm/barrier.h
// asm-generic/bitops/__ffs.h
// asm-generic/bitops/ffz.h
// asm-generic/bitops/fls.h
// asm-generic/bitops/__fls.h
// asm-generic/bitops/fls64.h
// asm-generic/bitops/sched.h
// asm-generic/bitops/ffs.h
// asm-generic/bitops/hweight.h
// asm-generic/bitops/lock.h
// asm-generic/bitops/atomic.h
// asm-generic/bitops/non-atomic.h
// asm-generic/bitops/le.h
// asm-generic/bitops/ext2-atomic.h


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
