/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated headers.
unsafe extern "C" {
    fn irq_fpu_usable() -> bool;
}

/*
 * may_use_simd - whether it is allowable at this time to issue SIMD
 *                instructions or access the SIMD register file
 */
#[must_use]
#[inline]
fn may_use_simd() -> bool {
    unsafe { irq_fpu_usable() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
