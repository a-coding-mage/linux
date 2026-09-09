/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C header guard: _ASM_BITOPS_H

// Dependency: linux/compiler.h
// This header may only be included through linux/bitops.h.

// Dependency: asm/barrier.h

// CONFIG_32BIT_REDUCED selects the asm-generic implementations of:
// ffs, fls, __ffs, and __fls.
// Otherwise (CONFIG_32BIT_STANDARD || CONFIG_64BIT), the builtin
// asm-generic implementations are selected.

// Dependencies supplied by other headers:
// asm-generic/bitops/ffz.h
// asm-generic/bitops/fls64.h
// asm-generic/bitops/sched.h
// asm-generic/bitops/hweight.h
// asm-generic/bitops/atomic.h
// asm-generic/bitops/non-atomic.h
// asm-generic/bitops/lock.h
// asm-generic/bitops/le.h
// asm-generic/bitops/ext2-atomic.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
