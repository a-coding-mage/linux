/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copied from the kernel sources:
 *
 * Copyright (C) 1999, 2000  Niibe Yutaka  &  Kaz Kojima
 * Copyright (C) 2002 Paul Mundt
 */

/*
 * A brief note on ctrl_barrier(), the control register write barrier.
 *
 * Legacy SH cores typically require a sequence of 8 nops after
 * modification of a control register in order for the changes to take
 * effect. On newer cores (like the sh4a and sh5) this is accomplished
 * with icbi.
 *
 * Also note that on sh4a in the icbi case we can forego a synco for the
 * write barrier, as it's not necessary for control registers.
 *
 * Historically we have only done this type of barrier for the MMUCR, but
 * it's also necessary for the CCR, so we make it generic here instead.
 */

// C preprocessor condition preserved from source:
// #if defined(__SH4A__)
#[cfg(__SH4A__)]
#[inline]
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!("synco", options(nostack, preserves_flags));
    }
}

#[cfg(__SH4A__)]
#[inline]
pub unsafe fn rmb() {
    unsafe {
        mb();
    }
}

#[cfg(__SH4A__)]
#[inline]
pub unsafe fn wmb() {
    unsafe {
        mb();
    }
}

// Dependency intent preserved from source:
// #include <asm-generic/barrier.h>

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
