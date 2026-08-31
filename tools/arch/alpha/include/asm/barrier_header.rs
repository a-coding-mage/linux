/* SPDX-License-Identifier: GPL-2.0 */

// Header guard omitted: __TOOLS_LINUX_ASM_ALPHA_BARRIER_H.

#[inline(always)]
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!("mb", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn rmb() {
    unsafe {
        core::arch::asm!("mb", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn wmb() {
    unsafe {
        core::arch::asm!("wmb", options(nostack, preserves_flags));
    }
}
