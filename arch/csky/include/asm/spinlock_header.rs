/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <asm/qspinlock.h>.
// Dependency provided by <asm/qrwlock.h>.

// Equivalent to the C macro:
// #define smp_mb__after_spinlock() smp_mb()
unsafe extern "C" {
    fn smp_mb();
}

#[inline]
pub unsafe fn smp_mb__after_spinlock() {
    unsafe {
        smp_mb();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
