/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// C header guard: _ASM_ARC_ATOMIC_H
// The original declarations are available only when __ASSEMBLER__ is not
// defined.

// Dependencies supplied by other headers:
// linux/types.h, linux/compiler.h, asm/cmpxchg.h, asm/barrier.h, asm/smp.h

/// Read the counter of an atomic value once.
#[macro_export]
macro_rules! arch_atomic_read {
    ($v:expr) => {
        READ_ONCE!((*$v).counter)
    };
}

// When CONFIG_ARC_HAS_LLSC is enabled, the implementation is supplied by
// asm/atomic-llsc.h; otherwise it is supplied by asm/atomic-spinlock.h.

/*
 * 64-bit atomics
 */
// When CONFIG_GENERIC_ATOMIC64 is enabled, the implementation is supplied by
// asm-generic/atomic64.h; otherwise it is supplied by asm/atomic64-arcv2.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
