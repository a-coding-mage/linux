/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/compiler_attributes.h, linux/preempt.h, linux/sched.h, linux/types.h

/*
 * may_use_simd - whether it is allowable at this time to issue SIMD
 *                instructions or access the SIMD register file
 *
 * As architectures typically don't preserve the SIMD register file when
 * taking an interrupt, !in_interrupt() should be a reasonable default.
 */
#[must_use]
#[inline]
pub fn may_use_simd() -> bool {
    unsafe { !in_interrupt() }
}

// External dependency declared by linux/preempt.h or linux/sched.h.
unsafe extern "C" {
    fn in_interrupt() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
