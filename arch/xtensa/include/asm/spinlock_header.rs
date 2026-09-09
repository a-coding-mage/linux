/*
 * include/asm-xtensa/spinlock.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// C dependencies: <asm/barrier.h>, <asm/qspinlock.h>, and <asm/qrwlock.h>.

/// Equivalent of the C `smp_mb__after_spinlock()` macro.
#[macro_export]
macro_rules! smp_mb__after_spinlock {
    () => {
        smp_mb!()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
