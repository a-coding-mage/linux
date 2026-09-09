/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding architecture implementation:
// <asm/cpufeatures.h>, <asm/alternative.h>, and <asm-generic/barrier.h>.

/*
 * Force strict CPU ordering.
 * And yes, this is required on UP too when we're talking
 * to devices.
 */

#[cfg(feature = "CONFIG_X86_32")]
#[inline(always)]
pub unsafe fn mb() {
    // The C implementation uses `alternative` to select mfence when
    // X86_FEATURE_XMM2 is available; otherwise it uses this locked add.
    core::arch::asm!("lock addl $0, 0(%esp)", options(att_syntax, preserves_flags));
}

#[cfg(feature = "CONFIG_X86_32")]
#[inline(always)]
pub unsafe fn rmb() {
    // The C implementation uses `alternative` to select lfence when
    // X86_FEATURE_XMM2 is available; otherwise it uses this locked add.
    core::arch::asm!("lock addl $0, 0(%esp)", options(att_syntax, preserves_flags));
}

#[cfg(feature = "CONFIG_X86_32")]
#[inline(always)]
pub unsafe fn wmb() {
    // The C implementation uses `alternative` to select sfence when
    // X86_FEATURE_XMM is available; otherwise it uses this locked add.
    core::arch::asm!("lock addl $0, 0(%esp)", options(att_syntax, preserves_flags));
}

#[cfg(not(feature = "CONFIG_X86_32"))]
#[inline(always)]
pub unsafe fn mb() {
    core::arch::asm!("mfence", options(nostack, preserves_flags));
}

#[cfg(not(feature = "CONFIG_X86_32"))]
#[inline(always)]
pub unsafe fn rmb() {
    core::arch::asm!("lfence", options(nostack, preserves_flags));
}

#[cfg(not(feature = "CONFIG_X86_32"))]
#[inline(always)]
pub unsafe fn wmb() {
    core::arch::asm!("sfence", options(nostack, preserves_flags));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
