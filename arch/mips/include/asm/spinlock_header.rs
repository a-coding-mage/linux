/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1999, 2000, 06 Ralf Baechle (ralf@linux-mips.org)
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

// Dependency supplied by asm/processor.h.
extern "C" {
    pub fn mmiowb();
}

// Dependency supplied by asm-generic/qspinlock_types.h.
#[repr(C)]
pub struct qspinlock {
    pub locked: u8,
}

// Dependency supplied by asm-generic/qspinlock_types.h.
extern "C" {
    pub fn smp_store_release(ptr: *mut u8, value: u8);
}

// The C self-referential macro `queued_spin_release queued_spin_release`
// preserves the existing symbol name.

/**
 * queued_spin_release - release a queued spinlock
 * @lock : Pointer to queued spinlock structure
 */
#[inline]
pub unsafe fn queued_spin_release(lock: *mut qspinlock) {
    /* This could be optimised with ARCH_HAS_MMIOWB */
    mmiowb();
    smp_store_release(&mut (*lock).locked, 0);
}

// Declarations from asm/qspinlock.h and asm/qrwlock.h are supplied by those
// dependencies and are intentionally not redefined here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
