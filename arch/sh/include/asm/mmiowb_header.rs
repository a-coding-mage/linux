/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm/barrier.h> supplies the write-memory-barrier operation.
extern "C" {
    fn wmb();
}

/* synco on SH-4A, otherwise a nop */
#[inline]
pub unsafe fn mmiowb() {
    wmb();
}

// Dependency: <asm-generic/mmiowb.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
