/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/cleanup.h, linux/compiler_attributes.h, linux/preempt.h,
// linux/types.h, and asm/neon.h.

extern "C" {
    fn in_hardirq() -> bool;
    fn irqs_disabled() -> bool;
    fn kernel_neon_begin();
    fn kernel_neon_end();
}

#[inline]
pub unsafe fn may_use_simd() -> bool {
    cfg!(CONFIG_KERNEL_MODE_NEON) && !in_hardirq() && !irqs_disabled()
}

// Equivalent of DEFINE_LOCK_GUARD_0(ksimd, kernel_neon_begin(), kernel_neon_end()).
// The lock-guard machinery is supplied by linux/cleanup.h.

#[macro_export]
macro_rules! scoped_ksimd {
    () => {
        scoped_guard!(ksimd)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
