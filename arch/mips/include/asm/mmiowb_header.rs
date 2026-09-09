/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <asm/barrier.h>.
unsafe extern "C" {
    fn wmb();
}

// #define mmiowb() wmb()
#[inline]
pub unsafe fn mmiowb() {
    wmb();
}

// Declarations from <asm-generic/mmiowb.h> are supplied by the corresponding
// translated dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
